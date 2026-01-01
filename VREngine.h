// VREngine.h - Core VR Engine Header
#pragma once

#include <memory>
#include <vector>
#include <string>
#include <unordered_map>
#include <functional>

// Forward declarations
struct GLFWwindow;

namespace VRPlatform {

// ============================================
// VR Hand Tracking System
// ============================================

enum class HandType {
    LEFT,
    RIGHT
};

struct Vector3 {
    float x, y, z;
    
    Vector3(float x = 0, float y = 0, float z = 0) : x(x), y(y), z(z) {}
    
    Vector3 operator+(const Vector3& other) const {
        return Vector3(x + other.x, y + other.y, z + other.z);
    }
    
    Vector3 operator*(float scalar) const {
        return Vector3(x * scalar, y * scalar, z * scalar);
    }
    
    float magnitude() const {
        return std::sqrt(x*x + y*y + z*z);
    }
};

struct Quaternion {
    float x, y, z, w;
    
    Quaternion(float x = 0, float y = 0, float z = 0, float w = 1)
        : x(x), y(y), z(z), w(w) {}
};

struct HandPose {
    Vector3 position;
    Quaternion rotation;
    bool isTracking;
    bool isGripping;
    bool isTriggerPressed;
    float gripStrength;
    float triggerStrength;
};

class VRHandTracker {
public:
    VRHandTracker();
    ~VRHandTracker();
    
    bool initialize();
    void update();
    
    HandPose getHandPose(HandType hand) const;
    bool isHandTracked(HandType hand) const;
    
    // Gesture detection
    bool isPinching(HandType hand) const;
    bool isPointing(HandType hand) const;
    bool isThumbsUp(HandType hand) const;
    
private:
    std::unordered_map<HandType, HandPose> handPoses;
    void* vrSystem; // OpenXR or SteamVR handle
};

// ============================================
// VR Rendering System
// ============================================

struct RenderTarget {
    unsigned int framebuffer;
    unsigned int colorTexture;
    unsigned int depthTexture;
    int width;
    int height;
};

class VRRenderer {
public:
    VRRenderer();
    ~VRRenderer();
    
    bool initialize(int width, int height);
    void shutdown();
    
    void beginFrame();
    void endFrame();
    
    void renderEye(int eyeIndex);
    void submitFrame();
    
    RenderTarget* getEyeRenderTarget(int eyeIndex);
    
    // Camera management
    void setViewMatrix(int eyeIndex, const float* matrix);
    void setProjectionMatrix(int eyeIndex, const float* matrix);
    
private:
    RenderTarget leftEye;
    RenderTarget rightEye;
    
    float viewMatrices[2][16];
    float projectionMatrices[2][16];
    
    void createRenderTarget(RenderTarget& target, int width, int height);
    void destroyRenderTarget(RenderTarget& target);
};

// ============================================
// VR Object System (for game creation)
// ============================================

enum class ObjectType {
    CUBE,
    SPHERE,
    CYLINDER,
    CUSTOM_MESH,
    TERRAIN,
    LIGHT,
    AUDIO_SOURCE,
    TRIGGER_VOLUME
};

class VRObject {
public:
    VRObject(ObjectType type);
    virtual ~VRObject() = default;
    
    // Transform
    void setPosition(const Vector3& pos) { position = pos; }
    void setRotation(const Quaternion& rot) { rotation = rot; }
    void setScale(const Vector3& scl) { scale = scl; }
    
    Vector3 getPosition() const { return position; }
    Quaternion getRotation() const { return rotation; }
    Vector3 getScale() const { return scale; }
    
    // Physics
    void setMass(float m) { mass = m; }
    void setGravityEnabled(bool enabled) { useGravity = enabled; }
    void setKinematic(bool kinematic) { isKinematic = kinematic; }
    
    // Interaction
    void setGrabbable(bool grabbable) { isGrabbable = grabbable; }
    void setInteractable(bool interactable) { isInteractable = interactable; }
    
    // Scripting hooks
    void onGrabbed(std::function<void()> callback) { onGrabbedCallback = callback; }
    void onReleased(std::function<void()> callback) { onReleasedCallback = callback; }
    void onCollision(std::function<void(VRObject*)> callback) { onCollisionCallback = callback; }
    
    virtual void update(float deltaTime);
    virtual void render();
    
protected:
    ObjectType type;
    Vector3 position;
    Quaternion rotation;
    Vector3 scale;
    
    // Physics properties
    float mass;
    bool useGravity;
    bool isKinematic;
    
    // Interaction
    bool isGrabbable;
    bool isInteractable;
    bool isGrabbed;
    
    // Callbacks
    std::function<void()> onGrabbedCallback;
    std::function<void()> onReleasedCallback;
    std::function<void(VRObject*)> onCollisionCallback;
    
    unsigned int meshId;
    unsigned int materialId;
};

// ============================================
// VR Scene Manager
// ============================================

class VRScene {
public:
    VRScene();
    ~VRScene();
    
    void addObject(std::shared_ptr<VRObject> object);
    void removeObject(std::shared_ptr<VRObject> object);
    
    void update(float deltaTime);
    void render(VRRenderer& renderer);
    
    std::vector<std::shared_ptr<VRObject>>& getObjects() { return objects; }
    
    // Raycasting for object selection
    VRObject* raycast(const Vector3& origin, const Vector3& direction, float maxDistance = 100.0f);
    
    void save(const std::string& filename);
    void load(const std::string& filename);
    
private:
    std::vector<std::shared_ptr<VRObject>> objects;
    void* physicsWorld; // Bullet physics world
};

// ============================================
// VR Game Creator Interface
// ============================================

class VRGameCreator {
public:
    VRGameCreator(VRScene& scene);
    ~VRGameCreator();
    
    void update(float deltaTime, VRHandTracker& handTracker);
    void render(VRRenderer& renderer);
    
    // Object manipulation
    void selectObject(VRObject* object);
    void deselectObject();
    VRObject* getSelectedObject() const { return selectedObject; }
    
    // Creation tools
    VRObject* spawnObject(ObjectType type, const Vector3& position);
    void deleteObject(VRObject* object);
    void duplicateObject(VRObject* object);
    
    // Transform tools
    void enableMoveTool();
    void enableRotateTool();
    void enableScaleTool();
    
    // Testing
    void enterPlayMode();
    void exitPlayMode();
    bool isInPlayMode() const { return playMode; }
    
    // Publishing
    void saveGame(const std::string& gameName);
    void publishGame(const std::string& gameName, const std::string& description);
    
private:
    VRScene& scene;
    VRObject* selectedObject;
    
    enum class Tool {
        SELECT,
        MOVE,
        ROTATE,
        SCALE,
        SPAWN
    };
    
    Tool currentTool;
    bool playMode;
    
    // UI state
    Vector3 toolGrabOffset;
    Vector3 originalPosition;
    Quaternion originalRotation;
    Vector3 originalScale;
    
    void updateObjectManipulation(VRHandTracker& handTracker);
    void renderGizmos(VRRenderer& renderer);
};

// ============================================
// Main VR Engine Class
// ============================================

class VREngine {
public:
    static VREngine& getInstance();
    
    bool initialize();
    void shutdown();
    
    void run();
    
    VRRenderer& getRenderer() { return renderer; }
    VRHandTracker& getHandTracker() { return handTracker; }
    VRScene& getCurrentScene() { return *currentScene; }
    VRGameCreator& getGameCreator() { return *gameCreator; }
    
    // Network connection
    void connectToServer(const std::string& serverAddress);
    void disconnect();
    bool isConnected() const { return connected; }
    
    // User session
    void setUserToken(const std::string& token) { userToken = token; }
    std::string getUserToken() const { return userToken; }
    
private:
    VREngine() = default;
    ~VREngine() = default;
    
    VREngine(const VREngine&) = delete;
    VREngine& operator=(const VREngine&) = delete;
    
    VRRenderer renderer;
    VRHandTracker handTracker;
    
    std::unique_ptr<VRScene> currentScene;
    std::unique_ptr<VRGameCreator> gameCreator;
    
    GLFWwindow* window;
    bool running;
    bool connected;
    std::string userToken;
    
    float lastFrameTime;
    
    void processInput();
    void update(float deltaTime);
    void render();
};

} // namespace VRPlatform
