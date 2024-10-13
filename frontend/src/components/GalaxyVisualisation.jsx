// src/components/GalaxyVisualization.jsx
import React, { useRef, useEffect } from 'react';
import { Canvas, useFrame, useThree, extend } from '@react-three/fiber';
import {
    OrbitControls,
    Stars,
    Text,
    Billboard,
} from '@react-three/drei';
import * as THREE from 'three';
import {
    EffectComposer,
    RenderPass,
    ShaderPass,
    UnrealBloomPass,
} from 'three-stdlib';

extend({ EffectComposer, RenderPass, ShaderPass, UnrealBloomPass });

function GalaxyVisualization() {
    // Fetch and store the galaxy data
    const [systems, setSystems] = React.useState([]);

    useEffect(() => {
        fetch('http://localhost:8080/systems')
            .then((response) => response.json())
            .then((data) => {
                setSystems(data);
            })
            .catch((error) => console.error('Error fetching galaxy data:', error));
    }, []);

    return (
        <Canvas
            style={{ width: '100%', height: '400px' }}
            camera={{ position: [0, 0, 5], fov: 75 }}
            gl={{ antialias: true }}
            onCreated={({ gl }) => {
                gl.setClearColor('black');
            }}
        >
            <Scene systems={systems} />
        </Canvas>
    );
}

function Scene({ systems }) {
    const { scene, camera, gl } = useThree();

    // Set up render targets
    const bloomLayer = new THREE.Layers();
    bloomLayer.set(1); // Layer 1 for bloom

    const materials = {};
    const darkMaterial = new THREE.MeshBasicMaterial({ color: 'black' });

    const bloomComposer = useRef();
    const finalComposer = useRef();

    useEffect(() => {
        const renderScene = new RenderPass(scene, camera);

        const bloomPass = new UnrealBloomPass(
            new THREE.Vector2(window.innerWidth, window.innerHeight),
            1.5, // strength
            0.4, // radius
            0.85 // threshold
        );

        bloomPass.renderToScreen = false;

        const bloomComposerInstance = new EffectComposer(gl);
        bloomComposerInstance.addPass(renderScene);
        bloomComposerInstance.addPass(bloomPass);
        bloomComposer.current = bloomComposerInstance;

        const finalPass = new ShaderPass(
            new THREE.ShaderMaterial({
                uniforms: {
                    baseTexture: { value: null },
                    bloomTexture: { value: bloomComposerInstance.renderTarget2.texture },
                },
                vertexShader: vertexShader(),
                fragmentShader: fragmentShader(),
                defines: {},
            }),
            'baseTexture'
        );
        finalPass.needsSwap = true;

        const finalComposerInstance = new EffectComposer(gl);
        finalComposerInstance.addPass(renderScene);
        finalComposerInstance.addPass(finalPass);
        finalComposer.current = finalComposerInstance;
    }, [scene, camera, gl]);

    useFrame(() => {
        // First render pass: render bloom layer
        scene.traverse((obj) => {
            if (obj.isMesh && bloomLayer.test(obj.layers) === false) {
                materials[obj.uuid] = obj.material;
                obj.material = darkMaterial;
            }
        });

        bloomComposer.current.render();

        // Restore materials
        scene.traverse((obj) => {
            if (materials[obj.uuid]) {
                obj.material = materials[obj.uuid];
                delete materials[obj.uuid];
            }
        });

        // Second render pass: render entire scene
        finalComposer.current.render();
    }, 1);

    return (
        <>
            {/* Lights and Controls */}
            <ambientLight intensity={0.5} />
            <pointLight position={[10, 10, 10]} />
            <OrbitControls />
            <Stars
                radius={100}
                depth={50}
                count={5000}
                factor={4}
                saturation={0}
                fade
            />

            {/* Star Systems */}
            {systems.map((system) => (
                <StarSystem key={system.id} system={system} bloomLayer={bloomLayer} />
            ))}
        </>
    );
}

function StarSystem({ system, bloomLayer }) {
    const mesh = useRef();
    const textRef = useRef();
    const [hovered, setHovered] = React.useState(false);
    const { camera } = useThree();

    // Determine star color based on temperature
    const color = getStarColor(system.star_temperature);

    // Make text always face the camera
    useFrame(() => {
        if (textRef.current) {
            textRef.current.quaternion.copy(camera.quaternion);
        }
    });

    // Assign bloom layer to the star mesh
    useEffect(() => {
        if (mesh.current) {
            mesh.current.layers.enable(1); // Enable layer 1 for bloom
        }
    }, []);

    return (
        <group position={[system.x, system.y, system.z]}>
            <mesh
                ref={mesh}
                onPointerOver={() => setHovered(true)}
                onPointerOut={() => setHovered(false)}
            >
                <sphereGeometry args={[0.01 + system.star_radius / 1000, 16, 16]} />
                <meshBasicMaterial color={color} />
            </mesh>

                <Text
                    ref={textRef}
                    position={[0, 0.12, 0]}
                    fontSize={0.05}
                    color="white"
                    anchorX="center"
                    anchorY="middle"
                >
                    {system.name}
                </Text>
        </group>
    );
}

function getStarColor(temperature) {
    if (temperature < 3500) return '#cc6666'; // Cool red stars
    if (temperature < 5000) return '#ffa500'; // Orange
    if (temperature < 6000) return '#ffff00'; // Yellow
    if (temperature < 10000) return '#ffffff'; // White
    if (temperature < 30000) return '#aaaaff'; // Blue-white
    return '#0000ff'; // Hot blue stars
}

// Vertex shader for the final pass
function vertexShader() {
    return `
    varying vec2 vUv;
    void main() {
      vUv = uv;
      gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );
    }
  `;
}

// Fragment shader for the final pass
function fragmentShader() {
    return `
    uniform sampler2D baseTexture;
    uniform sampler2D bloomTexture;

    varying vec2 vUv;

    void main() {
      gl_FragColor = ( texture2D( baseTexture, vUv ) + texture2D( bloomTexture, vUv ) );
    }
  `;
}

export default GalaxyVisualization;
