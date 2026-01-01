// multiplayer_server.rs - VR Multiplayer Game Server
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

// ============================================
// Network Protocol Data Structures
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerTransform {
    pub position: Vector3,
    pub rotation: Quaternion,
    pub head_position: Vector3,
    pub head_rotation: Quaternion,
    pub left_hand_position: Vector3,
    pub left_hand_rotation: Quaternion,
    pub right_hand_position: Vector3,
    pub right_hand_rotation: Quaternion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub player_id: String,
    pub username: String,
    pub transform: PlayerTransform,
    pub avatar_url: Option<String>,
    pub is_talking: bool,
    pub custom_data: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GameMessage {
    // Connection
    Connect {
        token: String,
        game_id: String,
    },
    Disconnect {
        player_id: String,
    },
    
    // Player updates
    PlayerJoined {
        player: PlayerState,
    },
    PlayerLeft {
        player_id: String,
    },
    PlayerUpdate {
        player_id: String,
        transform: PlayerTransform,
    },
    
    // Game state
    ObjectSpawned {
        object_id: String,
        object_type: String,
        position: Vector3,
        rotation: Quaternion,
    },
    ObjectMoved {
        object_id: String,
        position: Vector3,
        rotation: Quaternion,
    },
    ObjectDestroyed {
        object_id: String,
    },
    ObjectGrabbed {
        object_id: String,
        player_id: String,
    },
    ObjectReleased {
        object_id: String,
    },
    
    // Voice chat
    VoiceData {
        player_id: String,
        audio_data: Vec<u8>,
    },
    
    // Custom game events
    CustomEvent {
        event_name: String,
        data: String,
    },
    
    // Server responses
    Error {
        message: String,
    },
    Success {
        message: String,
    },
}

// ============================================
// Game Room Management
// ============================================

#[derive(Debug, Clone)]
pub struct GameRoom {
    pub room_id: String,
    pub game_id: String,
    pub host_id: String,
    pub max_players: usize,
    pub players: Vec<PlayerState>,
    pub created_at: i64,
    pub is_public: bool,
    pub game_state: HashMap<String, String>,
}

impl GameRoom {
    pub fn new(room_id: String, game_id: String, host_id: String, max_players: usize) -> Self {
        Self {
            room_id,
            game_id,
            host_id,
            max_players,
            players: Vec::new(),
            created_at: chrono::Utc::now().timestamp(),
            is_public: true,
            game_state: HashMap::new(),
        }
    }
    
    pub fn add_player(&mut self, player: PlayerState) -> bool {
        if self.players.len() >= self.max_players {
            return false;
        }
        
        self.players.push(player);
        true
    }
    
    pub fn remove_player(&mut self, player_id: &str) {
        self.players.retain(|p| p.player_id != player_id);
    }
    
    pub fn get_player(&self, player_id: &str) -> Option<&PlayerState> {
        self.players.iter().find(|p| p.player_id == player_id)
    }
    
    pub fn update_player(&mut self, player_id: &str, transform: PlayerTransform) {
        if let Some(player) = self.players.iter_mut().find(|p| p.player_id == player_id) {
            player.transform = transform;
        }
    }
    
    pub fn is_full(&self) -> bool {
        self.players.len() >= self.max_players
    }
    
    pub fn player_count(&self) -> usize {
        self.players.len()
    }
}

// ============================================
// Multiplayer Server
// ============================================

pub struct MultiplayerServer {
    rooms: Arc<Mutex<HashMap<String, GameRoom>>>,
    player_to_room: Arc<Mutex<HashMap<String, String>>>, // player_id -> room_id
    connections: Arc<Mutex<HashMap<String, SocketAddr>>>, // player_id -> address
}

impl MultiplayerServer {
    pub fn new() -> Self {
        println!("[MultiplayerServer] Initializing multiplayer server...");
        
        Self {
            rooms: Arc::new(Mutex::new(HashMap::new())),
            player_to_room: Arc::new(Mutex::new(HashMap::new())),
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Create a new game room
    pub fn create_room(&self, game_id: String, host_id: String, max_players: usize) -> String {
        let room_id = format!("room_{}", uuid::Uuid::new_v4());
        let room = GameRoom::new(room_id.clone(), game_id, host_id, max_players);
        
        let mut rooms = self.rooms.lock().unwrap();
        rooms.insert(room_id.clone(), room);
        
        println!("[MultiplayerServer] Room created: {}", room_id);
        
        room_id
    }
    
    /// Join an existing room
    pub fn join_room(&self, room_id: &str, player: PlayerState) -> Result<(), String> {
        let mut rooms = self.rooms.lock().unwrap();
        
        if let Some(room) = rooms.get_mut(room_id) {
            if room.is_full() {
                return Err("Room is full".to_string());
            }
            
            let player_id = player.player_id.clone();
            
            if room.add_player(player) {
                // Update player-to-room mapping
                drop(rooms); // Release the lock
                let mut player_to_room = self.player_to_room.lock().unwrap();
                player_to_room.insert(player_id.clone(), room_id.to_string());
                
                println!("[MultiplayerServer] Player {} joined room {}", player_id, room_id);
                Ok(())
            } else {
                Err("Failed to add player to room".to_string())
            }
        } else {
            Err("Room not found".to_string())
        }
    }
    
    /// Leave a room
    pub fn leave_room(&self, player_id: &str) -> Option<String> {
        let mut player_to_room = self.player_to_room.lock().unwrap();
        
        if let Some(room_id) = player_to_room.remove(player_id) {
            drop(player_to_room);
            
            let mut rooms = self.rooms.lock().unwrap();
            if let Some(room) = rooms.get_mut(&room_id) {
                room.remove_player(player_id);
                
                // Delete room if empty
                if room.player_count() == 0 {
                    rooms.remove(&room_id);
                    println!("[MultiplayerServer] Room {} deleted (empty)", room_id);
                } else {
                    println!("[MultiplayerServer] Player {} left room {}", player_id, room_id);
                }
                
                return Some(room_id);
            }
        }
        
        None
    }
    
    /// Update player transform
    pub fn update_player(&self, player_id: &str, transform: PlayerTransform) -> Option<String> {
        let player_to_room = self.player_to_room.lock().unwrap();
        
        if let Some(room_id) = player_to_room.get(player_id) {
            let room_id = room_id.clone();
            drop(player_to_room);
            
            let mut rooms = self.rooms.lock().unwrap();
            if let Some(room) = rooms.get_mut(&room_id) {
                room.update_player(player_id, transform);
                return Some(room_id);
            }
        }
        
        None
    }
    
    /// Get all players in a room
    pub fn get_room_players(&self, room_id: &str) -> Vec<PlayerState> {
        let rooms = self.rooms.lock().unwrap();
        
        if let Some(room) = rooms.get(room_id) {
            room.players.clone()
        } else {
            Vec::new()
        }
    }
    
    /// Find available rooms for a game
    pub fn find_rooms(&self, game_id: &str) -> Vec<(String, usize, usize)> {
        let rooms = self.rooms.lock().unwrap();
        
        rooms.values()
            .filter(|room| room.game_id == game_id && room.is_public && !room.is_full())
            .map(|room| (room.room_id.clone(), room.player_count(), room.max_players))
            .collect()
    }
    
    /// Get room info
    pub fn get_room(&self, room_id: &str) -> Option<GameRoom> {
        let rooms = self.rooms.lock().unwrap();
        rooms.get(room_id).cloned()
    }
    
    /// Broadcast message to all players in a room
    pub fn broadcast_to_room(&self, room_id: &str, message: GameMessage, exclude_player: Option<&str>) {
        let rooms = self.rooms.lock().unwrap();
        
        if let Some(room) = rooms.get(room_id) {
            for player in &room.players {
                if let Some(excluded) = exclude_player {
                    if player.player_id == excluded {
                        continue;
                    }
                }
                
                // In real implementation, send message over network
                println!("[MultiplayerServer] Broadcasting to player: {}", player.player_id);
            }
        }
    }
    
    /// Get server statistics
    pub fn get_stats(&self) -> ServerStats {
        let rooms = self.rooms.lock().unwrap();
        let connections = self.connections.lock().unwrap();
        
        let total_players: usize = rooms.values().map(|r| r.player_count()).sum();
        
        ServerStats {
            total_rooms: rooms.len(),
            total_players,
            active_connections: connections.len(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ServerStats {
    pub total_rooms: usize,
    pub total_players: usize,
    pub active_connections: usize,
}

// ============================================
// Voice Chat System
// ============================================

pub struct VoiceChatServer {
    active_channels: Arc<Mutex<HashMap<String, Vec<String>>>>, // room_id -> [player_ids]
}

impl VoiceChatServer {
    pub fn new() -> Self {
        Self {
            active_channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn join_voice_channel(&self, room_id: &str, player_id: &str) {
        let mut channels = self.active_channels.lock().unwrap();
        
        channels.entry(room_id.to_string())
            .or_insert_with(Vec::new)
            .push(player_id.to_string());
        
        println!("[VoiceChat] Player {} joined voice channel in room {}", player_id, room_id);
    }
    
    pub fn leave_voice_channel(&self, room_id: &str, player_id: &str) {
        let mut channels = self.active_channels.lock().unwrap();
        
        if let Some(players) = channels.get_mut(room_id) {
            players.retain(|p| p != player_id);
            
            if players.is_empty() {
                channels.remove(room_id);
            }
        }
        
        println!("[VoiceChat] Player {} left voice channel", player_id);
    }
    
    pub fn broadcast_audio(&self, room_id: &str, sender_id: &str, audio_data: Vec<u8>) {
        let channels = self.active_channels.lock().unwrap();
        
        if let Some(players) = channels.get(room_id) {
            for player_id in players {
                if player_id != sender_id {
                    // Send audio data to player
                    println!("[VoiceChat] Sending audio from {} to {}", sender_id, player_id);
                }
            }
        }
    }
}

// ============================================
// Matchmaking System
// ============================================

pub struct MatchmakingService {
    queue: Arc<Mutex<HashMap<String, Vec<String>>>>, // game_id -> [player_ids]
}

impl MatchmakingService {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn join_queue(&self, game_id: &str, player_id: &str) {
        let mut queue = self.queue.lock().unwrap();
        
        queue.entry(game_id.to_string())
            .or_insert_with(Vec::new)
            .push(player_id.to_string());
        
        println!("[Matchmaking] Player {} joined queue for game {}", player_id, game_id);
    }
    
    pub fn leave_queue(&self, game_id: &str, player_id: &str) {
        let mut queue = self.queue.lock().unwrap();
        
        if let Some(players) = queue.get_mut(game_id) {
            players.retain(|p| p != player_id);
        }
    }
    
    pub fn find_match(&self, game_id: &str, required_players: usize) -> Option<Vec<String>> {
        let mut queue = self.queue.lock().unwrap();
        
        if let Some(players) = queue.get_mut(game_id) {
            if players.len() >= required_players {
                let matched: Vec<String> = players.drain(0..required_players).collect();
                println!("[Matchmaking] Match found for game {}: {} players", game_id, matched.len());
                return Some(matched);
            }
        }
        
        None
    }
}

// ============================================
// Tests
// ============================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_room_creation() {
        let server = MultiplayerServer::new();
        let room_id = server.create_room("game123".to_string(), "player1".to_string(), 4);
        
        assert!(!room_id.is_empty());
        
        let room = server.get_room(&room_id);
        assert!(room.is_some());
    }
    
    #[test]
    fn test_player_join() {
        let server = MultiplayerServer::new();
        let room_id = server.create_room("game123".to_string(), "host".to_string(), 4);
        
        let player = PlayerState {
            player_id: "player1".to_string(),
            username: "TestPlayer".to_string(),
            transform: PlayerTransform {
                position: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                rotation: Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
                head_position: Vector3 { x: 0.0, y: 1.7, z: 0.0 },
                head_rotation: Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
                left_hand_position: Vector3 { x: -0.3, y: 1.2, z: 0.3 },
                left_hand_rotation: Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
                right_hand_position: Vector3 { x: 0.3, y: 1.2, z: 0.3 },
                right_hand_rotation: Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
            },
            avatar_url: None,
            is_talking: false,
            custom_data: HashMap::new(),
        };
        
        let result = server.join_room(&room_id, player);
        assert!(result.is_ok());
    }
}
