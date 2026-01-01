// auth_server.rs - User Authentication and Account Management System
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};

// ============================================
// Data Structures
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: i64,
    pub avatar_url: Option<String>,
    pub games_created: Vec<String>,
    pub games_played: Vec<String>,
    pub friends: Vec<String>,
    pub achievements: Vec<Achievement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub unlocked_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    pub token: Option<String>,
    pub user: Option<UserProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub games_created: Vec<String>,
    pub games_played: Vec<String>,
    pub friend_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,  // user id
    exp: i64,     // expiration time
    iat: i64,     // issued at
}

// ============================================
// Authentication Service
// ============================================

pub struct AuthService {
    users: Arc<Mutex<HashMap<String, User>>>,
    email_to_id: Arc<Mutex<HashMap<String, String>>>,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(jwt_secret: String) -> Self {
        println!("[AuthService] Initializing authentication service...");
        
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
            email_to_id: Arc::new(Mutex::new(HashMap::new())),
            jwt_secret,
        }
    }
    
    /// Sign up a new user
    pub fn signup(&self, request: SignupRequest) -> AuthResponse {
        println!("[AuthService] Signup request for: {}", request.username);
        
        // Validate input
        if request.username.len() < 3 {
            return AuthResponse {
                success: false,
                message: "Username must be at least 3 characters".to_string(),
                token: None,
                user: None,
            };
        }
        
        if !request.email.contains('@') {
            return AuthResponse {
                success: false,
                message: "Invalid email address".to_string(),
                token: None,
                user: None,
            };
        }
        
        if request.password.len() < 8 {
            return AuthResponse {
                success: false,
                message: "Password must be at least 8 characters".to_string(),
                token: None,
                user: None,
            };
        }
        
        // Check if email already exists
        {
            let email_map = self.email_to_id.lock().unwrap();
            if email_map.contains_key(&request.email) {
                return AuthResponse {
                    success: false,
                    message: "Email already registered".to_string(),
                    token: None,
                    user: None,
                };
            }
        }
        
        // Hash password
        let password_hash = match hash(&request.password, DEFAULT_COST) {
            Ok(h) => h,
            Err(e) => {
                eprintln!("[AuthService] Password hashing error: {}", e);
                return AuthResponse {
                    success: false,
                    message: "Internal server error".to_string(),
                    token: None,
                    user: None,
                };
            }
        };
        
        // Create user
        let user_id = format!("user_{}", uuid::Uuid::new_v4());
        let user = User {
            id: user_id.clone(),
            username: request.username.clone(),
            email: request.email.clone(),
            password_hash,
            created_at: Utc::now().timestamp(),
            avatar_url: None,
            games_created: Vec::new(),
            games_played: Vec::new(),
            friends: Vec::new(),
            achievements: Vec::new(),
        };
        
        // Store user
        {
            let mut users = self.users.lock().unwrap();
            users.insert(user_id.clone(), user.clone());
        }
        
        {
            let mut email_map = self.email_to_id.lock().unwrap();
            email_map.insert(request.email.clone(), user_id.clone());
        }
        
        // Generate JWT token
        let token = self.generate_token(&user_id);
        
        println!("[AuthService] User created successfully: {}", user.username);
        
        AuthResponse {
            success: true,
            message: "Account created successfully".to_string(),
            token: Some(token),
            user: Some(self.user_to_profile(&user)),
        }
    }
    
    /// Log in an existing user
    pub fn login(&self, request: LoginRequest) -> AuthResponse {
        println!("[AuthService] Login attempt for: {}", request.email);
        
        // Find user by email
        let user_id = {
            let email_map = self.email_to_id.lock().unwrap();
            match email_map.get(&request.email) {
                Some(id) => id.clone(),
                None => {
                    return AuthResponse {
                        success: false,
                        message: "Invalid email or password".to_string(),
                        token: None,
                        user: None,
                    };
                }
            }
        };
        
        // Get user
        let user = {
            let users = self.users.lock().unwrap();
            match users.get(&user_id) {
                Some(u) => u.clone(),
                None => {
                    return AuthResponse {
                        success: false,
                        message: "User not found".to_string(),
                        token: None,
                        user: None,
                    };
                }
            }
        };
        
        // Verify password
        match verify(&request.password, &user.password_hash) {
            Ok(valid) if valid => {
                let token = self.generate_token(&user.id);
                
                println!("[AuthService] Login successful: {}", user.username);
                
                AuthResponse {
                    success: true,
                    message: "Login successful".to_string(),
                    token: Some(token),
                    user: Some(self.user_to_profile(&user)),
                }
            }
            _ => {
                println!("[AuthService] Login failed: Invalid password");
                
                AuthResponse {
                    success: false,
                    message: "Invalid email or password".to_string(),
                    token: None,
                    user: None,
                }
            }
        }
    }
    
    /// Verify a JWT token and return user ID
    pub fn verify_token(&self, token: &str) -> Option<String> {
        let validation = Validation::default();
        
        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &validation,
        ) {
            Ok(data) => Some(data.claims.sub),
            Err(e) => {
                eprintln!("[AuthService] Token verification failed: {}", e);
                None
            }
        }
    }
    
    /// Get user profile by ID
    pub fn get_user(&self, user_id: &str) -> Option<UserProfile> {
        let users = self.users.lock().unwrap();
        users.get(user_id).map(|u| self.user_to_profile(u))
    }
    
    /// Update user profile
    pub fn update_user(&self, user_id: &str, avatar_url: Option<String>) -> bool {
        let mut users = self.users.lock().unwrap();
        
        if let Some(user) = users.get_mut(user_id) {
            user.avatar_url = avatar_url;
            println!("[AuthService] User profile updated: {}", user_id);
            true
        } else {
            false
        }
    }
    
    /// Add friend
    pub fn add_friend(&self, user_id: &str, friend_id: &str) -> bool {
        let mut users = self.users.lock().unwrap();
        
        if let Some(user) = users.get_mut(user_id) {
            if !user.friends.contains(&friend_id.to_string()) {
                user.friends.push(friend_id.to_string());
                println!("[AuthService] Friend added: {} -> {}", user_id, friend_id);
                return true;
            }
        }
        
        false
    }
    
    /// Record game creation
    pub fn add_created_game(&self, user_id: &str, game_id: &str) {
        let mut users = self.users.lock().unwrap();
        
        if let Some(user) = users.get_mut(user_id) {
            user.games_created.push(game_id.to_string());
            println!("[AuthService] Game created recorded: {} by {}", game_id, user_id);
        }
    }
    
    /// Record game play
    pub fn add_played_game(&self, user_id: &str, game_id: &str) {
        let mut users = self.users.lock().unwrap();
        
        if let Some(user) = users.get_mut(user_id) {
            if !user.games_played.contains(&game_id.to_string()) {
                user.games_played.push(game_id.to_string());
                println!("[AuthService] Game played recorded: {} by {}", game_id, user_id);
            }
        }
    }
    
    /// Unlock achievement
    pub fn unlock_achievement(&self, user_id: &str, achievement_id: &str, name: &str, description: &str) {
        let mut users = self.users.lock().unwrap();
        
        if let Some(user) = users.get_mut(user_id) {
            let achievement = Achievement {
                id: achievement_id.to_string(),
                name: name.to_string(),
                description: description.to_string(),
                unlocked_at: Utc::now().timestamp(),
            };
            
            user.achievements.push(achievement);
            println!("[AuthService] Achievement unlocked: {} for {}", name, user_id);
        }
    }
    
    // Private helper methods
    
    fn generate_token(&self, user_id: &str) -> String {
        let expiration = Utc::now()
            .checked_add_signed(Duration::days(30))
            .expect("Valid timestamp")
            .timestamp();
        
        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
            iat: Utc::now().timestamp(),
        };
        
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .unwrap()
    }
    
    fn user_to_profile(&self, user: &User) -> UserProfile {
        UserProfile {
            id: user.id.clone(),
            username: user.username.clone(),
            email: user.email.clone(),
            avatar_url: user.avatar_url.clone(),
            games_created: user.games_created.clone(),
            games_played: user.games_played.clone(),
            friend_count: user.friends.len(),
        }
    }
}

// ============================================
// Session Manager
// ============================================

pub struct SessionManager {
    active_sessions: Arc<Mutex<HashMap<String, String>>>, // token -> user_id
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            active_sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn create_session(&self, token: String, user_id: String) {
        let mut sessions = self.active_sessions.lock().unwrap();
        sessions.insert(token, user_id);
        println!("[SessionManager] Session created");
    }
    
    pub fn get_user_id(&self, token: &str) -> Option<String> {
        let sessions = self.active_sessions.lock().unwrap();
        sessions.get(token).cloned()
    }
    
    pub fn remove_session(&self, token: &str) {
        let mut sessions = self.active_sessions.lock().unwrap();
        sessions.remove(token);
        println!("[SessionManager] Session removed");
    }
    
    pub fn active_count(&self) -> usize {
        let sessions = self.active_sessions.lock().unwrap();
        sessions.len()
    }
}

// ============================================
// Example Usage / Tests
// ============================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_signup_and_login() {
        let auth = AuthService::new("test_secret_key".to_string());
        
        // Test signup
        let signup_req = SignupRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        
        let signup_resp = auth.signup(signup_req);
        assert!(signup_resp.success);
        assert!(signup_resp.token.is_some());
        
        // Test login
        let login_req = LoginRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        
        let login_resp = auth.login(login_req);
        assert!(login_resp.success);
        assert!(login_resp.token.is_some());
    }
    
    #[test]
    fn test_token_verification() {
        let auth = AuthService::new("test_secret_key".to_string());
        
        let signup_req = SignupRequest {
            username: "testuser2".to_string(),
            email: "test2@example.com".to_string(),
            password: "password123".to_string(),
        };
        
        let resp = auth.signup(signup_req);
        let token = resp.token.unwrap();
        
        let user_id = auth.verify_token(&token);
        assert!(user_id.is_some());
    }
}
