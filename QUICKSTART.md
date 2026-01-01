# VR Game Platform - Quick Start Guide

## 🚀 Getting Started

### Prerequisites

**For C++ VR Engine:**
- C++17 compatible compiler (GCC 9+, Clang 10+, MSVC 2019+)
- CMake 3.15 or higher
- OpenGL 4.5+ or Vulkan 1.2+
- VR Headset with OpenXR or SteamVR support

**For Rust Backend:**
- Rust 1.70 or higher
- PostgreSQL 14+ (for production)
- Redis (for caching)

**System Requirements:**
- 16 GB RAM minimum (32 GB recommended)
- NVIDIA GTX 1070 / AMD RX 580 or better
- 10 GB free disk space

---

## 📦 Installation

### 1. Clone the Repository

```bash
git clone https://github.com/yourorg/vr-game-platform.git
cd vr-game-platform
```

### 2. Build the C++ VR Engine

```bash
# Create build directory
mkdir build && cd build

# Configure with CMake
cmake .. -DCMAKE_BUILD_TYPE=Release

# Build
cmake --build . -j8

# Install (optional)
sudo cmake --install .
```

### 3. Build the Rust Backend

```bash
# Install dependencies
cargo build --release

# Run tests
cargo test

# Start the authentication server
cargo run --bin auth-server

# Start the game server (in another terminal)
cargo run --bin game-server
```

---

## 🎮 Running Your First VR Game

### Basic Usage

```bash
# Run the main VR application
./build/VRPlatformApp
```

This will:
1. Initialize the VR system
2. Create a sample scene with interactive objects
3. Launch the demo menu

### Menu Options

1. **Play Mode** - Experience the VR scene
2. **Creator Mode** - Edit and create games
3. **Multiplayer** - Connect to game servers
4. **Save/Load** - Manage your projects

---

## 👨‍💻 Creating Your First VR Game

### Example 1: Simple Interactive Scene

```cpp
#include "VREngine.h"

using namespace VRPlatform;

int main() {
    // Initialize engine
    auto& engine = VREngine::getInstance();
    engine.initialize();
    
    // Get the scene
    auto& scene = engine.getCurrentScene();
    
    // Add ground
    auto ground = std::make_shared<VRObject>(ObjectType::CUBE);
    ground->setPosition(Vector3(0, -1, 0));
    ground->setScale(Vector3(10, 0.1, 10));
    scene.addObject(ground);
    
    // Add interactive cube
    auto cube = std::make_shared<VRObject>(ObjectType::CUBE);
    cube->setPosition(Vector3(0, 1, -3));
    cube->setGrabbable(true);
    
    // Add interaction callbacks
    cube->onGrabbed([]() {
        std::cout << "Cube grabbed!" << std::endl;
    });
    
    scene.addObject(cube);
    
    // Run the game
    engine.run();
    
    return 0;
}
```

### Example 2: Using the Game Creator API

```cpp
#include "VREngine.h"

int main() {
    auto& engine = VREngine::getInstance();
    engine.initialize();
    
    auto& creator = engine.getGameCreator();
    
    // Spawn objects
    creator.spawnObject(ObjectType::SPHERE, Vector3(0, 2, -5));
    creator.spawnObject(ObjectType::CYLINDER, Vector3(2, 1, -5));
    
    // Enable move tool
    creator.enableMoveTool();
    
    // Save the game
    creator.saveGame("MyFirstGame");
    creator.publishGame("MyFirstGame", "My awesome VR creation!");
    
    return 0;
}
```

---

## 🌐 Setting Up User Authentication

### Start the Auth Server

```rust
// src/bin/auth_server_main.rs
use vr_game_platform::auth_server::*;

#[tokio::main]
async fn main() {
    let auth_service = AuthService::new("your-secret-key".to_string());
    
    // Example: Sign up a user
    let signup = SignupRequest {
        username: "player1".to_string(),
        email: "player1@example.com".to_string(),
        password: "securepass123".to_string(),
    };
    
    let response = auth_service.signup(signup);
    
    if response.success {
        println!("User created! Token: {}", response.token.unwrap());
    }
}
```

### Integrate with VR Client

```cpp
// In your VR app
auto& engine = VREngine::getInstance();

// Set user token after login
engine.setUserToken("jwt_token_here");

// Connect to multiplayer
engine.connectToServer("wss://yourserver.com");
```

---

## 🎨 Creating Custom VR Objects

### Define Custom Object Behavior

```cpp
class MyCustomObject : public VRObject {
public:
    MyCustomObject() : VRObject(ObjectType::CUSTOM_MESH) {
        health = 100;
    }
    
    void update(float deltaTime) override {
        // Custom update logic
        rotation += deltaTime * 0.5f; // Rotate over time
        
        VRObject::update(deltaTime);
    }
    
    void takeDamage(int amount) {
        health -= amount;
        if (health <= 0) {
            // Object destroyed
        }
    }
    
private:
    int health;
    float rotation;
};
```

---

## 🔌 Multiplayer Game Setup

### Server-Side (Rust)

```rust
use vr_game_platform::multiplayer_server::*;

let server = MultiplayerServer::new();

// Create a game room
let room_id = server.create_room(
    "game_123".to_string(),
    "host_player".to_string(),
    8  // max players
);

// Handle player join
let player = PlayerState {
    player_id: "player_456".to_string(),
    username: "CoolPlayer".to_string(),
    // ... other fields
};

server.join_room(&room_id, player).unwrap();
```

### Client-Side (C++)

```cpp
// Connect to server
engine.connectToServer("wss://game-server.com");

// Join a specific room
// (This would be handled through network messages)
```

---

## 🛠️ Advanced Features

### Adding Physics Interactions

```cpp
auto ball = std::make_shared<VRObject>(ObjectType::SPHERE);
ball->setMass(2.0f);
ball->setGravityEnabled(true);
ball->setPosition(Vector3(0, 5, -3));

// Add collision callback
ball->onCollision([](VRObject* other) {
    std::cout << "Ball collided with another object!" << std::endl;
});
```

### Implementing Game Logic with Lua

```lua
-- game_script.lua
function onStart()
    print("Game started!")
    score = 0
end

function onCubeGrabbed()
    score = score + 10
    print("Score: " .. score)
end

function onUpdate(deltaTime)
    -- Update game logic every frame
end
```

Load and execute from C++:

```cpp
// Load Lua script
luaL_dofile(L, "game_script.lua");

// Call Lua function
lua_getglobal(L, "onStart");
lua_call(L, 0, 0);
```

---

## 📊 Performance Optimization Tips

1. **Use Object Pooling**
   ```cpp
   // Reuse objects instead of creating new ones
   std::vector<std::shared_ptr<VRObject>> objectPool;
   ```

2. **Optimize Render Calls**
   - Use instanced rendering for multiple similar objects
   - Implement frustum culling
   - Use LOD (Level of Detail) systems

3. **Network Optimization**
   - Only send position updates when changed significantly
   - Compress network messages
   - Use prediction and interpolation

4. **Profile Your Game**
   ```cpp
   // Enable profiling
   engine.getRenderer().enableProfiling(true);
   ```

---

## 🐛 Debugging

### Enable Debug Mode

```cpp
// Enable debug visualization
#define VR_DEBUG_MODE
```

### Common Issues

**VR Headset Not Detected:**
- Ensure SteamVR or Oculus software is running
- Check USB and DisplayPort connections
- Update VR drivers

**Low Frame Rate:**
- Reduce graphics quality settings
- Limit number of physics objects
- Optimize scripts and callbacks

**Network Connection Failed:**
- Check firewall settings
- Verify server is running
- Check network credentials

---

## 📚 Next Steps

1. **Explore Examples** - Check the `/examples` directory
2. **Read Documentation** - See `/docs` for detailed API docs
3. **Join Community** - Discord server for support
4. **Contribute** - Submit PRs and report issues on GitHub

---

## 🆘 Getting Help

- **Documentation**: `/docs/API.md`
- **Examples**: `/examples`
- **Discord**: discord.gg/vrgameplatform
- **GitHub Issues**: github.com/yourorg/vr-game-platform/issues
- **Email**: support@vrgameplatform.com

---

## 📝 License

This project is licensed under the MIT License. See `LICENSE` file for details.

---

**Happy Creating! 🎮✨**
