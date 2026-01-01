// main.cpp - VR Game Platform Main Application
#include "VREngine.h"
#include <iostream>
#include <string>
#include <thread>
#include <chrono>

using namespace VRPlatform;

// ============================================
// Example: Simple VR Game Scene
// ============================================

void createExampleScene(VRScene& scene) {
    std::cout << "\n[Example] Creating sample VR scene...\n" << std::endl;
    
    // Add ground plane
    auto ground = std::make_shared<VRObject>(ObjectType::CUBE);
    ground->setPosition(Vector3(0, -1, 0));
    ground->setScale(Vector3(20, 0.1, 20));
    ground->setKinematic(true);
    scene.addObject(ground);
    std::cout << "  ✓ Ground plane added" << std::endl;
    
    // Add some cubes to interact with
    for (int i = 0; i < 5; i++) {
        auto cube = std::make_shared<VRObject>(ObjectType::CUBE);
        cube->setPosition(Vector3(i * 2.0f - 4.0f, 1.0f, -3.0f));
        cube->setScale(Vector3(0.5f, 0.5f, 0.5f));
        cube->setGrabbable(true);
        
        // Add grab callback
        cube->onGrabbed([i]() {
            std::cout << "  → Cube " << i << " grabbed!" << std::endl;
        });
        
        cube->onReleased([i]() {
            std::cout << "  → Cube " << i << " released!" << std::endl;
        });
        
        scene.addObject(cube);
    }
    std::cout << "  ✓ 5 interactive cubes added" << std::endl;
    
    // Add a sphere
    auto sphere = std::make_shared<VRObject>(ObjectType::SPHERE);
    sphere->setPosition(Vector3(0, 2, -5));
    sphere->setScale(Vector3(0.8f, 0.8f, 0.8f));
    sphere->setGrabbable(true);
    scene.addObject(sphere);
    std::cout << "  ✓ Interactive sphere added" << std::endl;
    
    std::cout << "\n[Example] Scene creation complete!\n" << std::endl;
}

// ============================================
// Demo Menu System
// ============================================

void displayMenu() {
    std::cout << "\n============================================" << std::endl;
    std::cout << "       VR GAME PLATFORM - DEMO MENU        " << std::endl;
    std::cout << "============================================" << std::endl;
    std::cout << "1. Play Mode - Experience the scene" << std::endl;
    std::cout << "2. Creator Mode - Edit and create" << std::endl;
    std::cout << "3. Multiplayer - Connect to server" << std::endl;
    std::cout << "4. Save Scene" << std::endl;
    std::cout << "5. Load Scene" << std::endl;
    std::cout << "6. Exit" << std::endl;
    std::cout << "============================================" << std::endl;
    std::cout << "Select option: ";
}

// ============================================
// Simulation Loop (for demo purposes)
// ============================================

void runSimulation(int seconds) {
    std::cout << "\n[Simulation] Running VR simulation for " << seconds << " seconds..." << std::endl;
    
    auto& engine = VREngine::getInstance();
    
    for (int i = 0; i < seconds; i++) {
        std::cout << "[Simulation] Frame " << i << " - Tracking hands, rendering scene..." << std::endl;
        
        // Simulate hand updates
        engine.getHandTracker().update();
        
        // Simulate scene update
        engine.getCurrentScene().update(0.016f); // ~60 FPS
        
        std::this_thread::sleep_for(std::chrono::seconds(1));
    }
    
    std::cout << "[Simulation] Simulation complete!\n" << std::endl;
}

// ============================================
// Main Application
// ============================================

int main(int argc, char** argv) {
    std::cout << R"(
    ╔════════════════════════════════════════════╗
    ║      VR GAME PLATFORM v1.0.0               ║
    ║      Play & Create in Virtual Reality      ║
    ╚════════════════════════════════════════════╝
    )" << std::endl;
    
    // Initialize the VR Engine
    auto& engine = VREngine::getInstance();
    
    if (!engine.initialize()) {
        std::cerr << "ERROR: Failed to initialize VR Engine!" << std::endl;
        return 1;
    }
    
    // Create an example scene
    createExampleScene(engine.getCurrentScene());
    
    // Demo menu loop
    bool running = true;
    while (running) {
        displayMenu();
        
        int choice;
        std::cin >> choice;
        
        switch (choice) {
            case 1: {
                std::cout << "\n[Play Mode] Starting game..." << std::endl;
                engine.getGameCreator().enterPlayMode();
                runSimulation(5);
                engine.getGameCreator().exitPlayMode();
                break;
            }
            
            case 2: {
                std::cout << "\n[Creator Mode] Entering game creator..." << std::endl;
                std::cout << "Creator tools active:" << std::endl;
                std::cout << "  - Select objects with trigger" << std::endl;
                std::cout << "  - Grab and move with grip button" << std::endl;
                std::cout << "  - Spawn new objects from menu" << std::endl;
                
                engine.getGameCreator().enableMoveTool();
                
                // Demonstrate spawning an object
                auto newObj = engine.getGameCreator().spawnObject(
                    ObjectType::CYLINDER,
                    Vector3(3, 1, -3)
                );
                std::cout << "\n  → Spawned a new cylinder!" << std::endl;
                
                runSimulation(5);
                break;
            }
            
            case 3: {
                std::cout << "\n[Multiplayer] Connecting to server..." << std::endl;
                engine.connectToServer("wss://vrplatform.example.com");
                
                if (engine.isConnected()) {
                    std::cout << "✓ Connected! You can now play with other users." << std::endl;
                    runSimulation(5);
                    engine.disconnect();
                }
                break;
            }
            
            case 4: {
                std::cout << "\n[Save] Enter game name: ";
                std::string gameName;
                std::cin.ignore();
                std::getline(std::cin, gameName);
                
                engine.getGameCreator().saveGame(gameName);
                std::cout << "✓ Game saved successfully!" << std::endl;
                break;
            }
            
            case 5: {
                std::cout << "\n[Load] Enter game name: ";
                std::string gameName;
                std::cin.ignore();
                std::getline(std::cin, gameName);
                
                engine.getCurrentScene().load("games/" + gameName + ".vrgame");
                std::cout << "✓ Game loaded successfully!" << std::endl;
                break;
            }
            
            case 6: {
                std::cout << "\n[Exit] Shutting down VR Platform..." << std::endl;
                running = false;
                break;
            }
            
            default: {
                std::cout << "\nInvalid option. Please try again." << std::endl;
                break;
            }
        }
        
        if (running) {
            std::cout << "\nPress Enter to continue...";
            std::cin.ignore();
            std::cin.get();
        }
    }
    
    // Cleanup
    engine.shutdown();
    
    std::cout << "\nThank you for using VR Game Platform!" << std::endl;
    std::cout << "Visit our website to share your creations!" << std::endl;
    
    return 0;
}
