# SistemaSolar-Rust

🌌 Sistema Solar — Software Renderer en Rust

Este proyecto es una simulación interactiva de un sistema solar hecho desde cero, utilizando un software renderer programado completamente en Rust.
Incluye cámara 3D controlable, planetas orbitando, un skybox de estrellas, una nave en primera persona y un efecto de “warp” para viajar por el sistema.

Todo corre sin usar OpenGL / Vulkan.
El dibujo es pixel por pixel en un framebuffer propio.

🚀 Características principales
✔ Renderizador por software

Framebuffer propio

Rasterización de triángulos

Transformaciones con matrices 4×4

Z-buffer básico

Pipeline de renderizado configurable

✔ Cámara 3D tipo “freecam”

Movimiento con WASD + mouse

Pitch y yaw

Subir y bajar (SPACE / CTRL)

Movimiento totalmente libre en 3D

✔ Nave en primera persona

Modelo .obj cargado desde assets/models/ship.obj

Sigue la orientación de la cámara

Se renderiza como parte del HUD 3D

✔ Sistema Solar propio

Sol en el centro (rotación lenta)

Planetas orbitando con sus velocidades y radios

Rotación propia de cada planeta

Malla de esfera low-poly para representar los cuerpos celestes

✔ Warp Jump (teletransporte animado)

Tecla 1, 2 o 3

Cámara acelera hacia un planeta seleccionado

Movimiento interpolado suavemente

✔ Skybox de estrellas

Generación procedural de estrellas

Distribución esférica

Rotan con la cámara pero no cambian de posición relativa

Se dibujan directamente en el framebuffer para mayor claridad

✔ Colisiones básicas

La cámara no puede atravesar planetas

Se ajusta automáticamente la posición si entra en el radio prohibido

📂 Estructura del proyecto
SistemaSolar/
├── Cargo.toml
├── README.md
├── assets/
│   ├── models/
│   │   └── ship.obj
│   └── textures/        (actualmente sin uso)
├── src/
│   ├── main.rs
│   ├── framebuffer.rs
│   ├── math/
│   ├── camera/
│   ├── scene/
│   ├── models/
│   ├── renderer/
│   ├── effects/
│   ├── physics/
│   └── input/


Cada módulo está separado para que sea fácil trabajar y extender el proyecto.

🎮 Controles
Acción	Tecla
Mover adelante/atrás	W / S
Mover izquierda/derecha	A / D
Subir / Bajar	SPACE / CTRL
Movimiento rápido	SHIFT
Girar cámara	Mouse
Warp al planeta 1	1
Warp al planeta 2	2
Warp al planeta 3	3
Salir	ESC
🎥 Video de demostración

He subido un video mostrando el funcionamiento completo del sistema, incluyendo:

movimiento de la cámara

orbitas

la nave

warp jump

estrellas y skybox


🔧 Cómo ejecutar el proyecto

Asegurate de tener Rust instalado:

rustup update


Luego ejecutá:

cargo run


La ventana abrirá a 1280×720, pero podés cambiarlo desde constantes en main.rs.

📝 Notas y limitaciones actuales

Los planetas solo usan color sólido por ahora (no se agregaron texturas).

El sol no tiene aún un efecto de glow más avanzado.

El renderer es funcional pero no está optimizado: no hay frustum culling ni multihilo.

La simulación es inventada, no corresponde a proporciones reales.

Aun así, el sistema funciona establemente sobre 30 FPS.

🔮 Posibles mejoras futuras

Texturas UV para planetas

Glow dinámico del sol

Lunas adicionales

Órbitas dibujadas visualmente

Corrección gamma

Skybox más elaborado con twinkling de estrellas

Shader de iluminación per-pixel (Lambert/Phong)

Soporte para mallas complejas además de esferas

📜 Licencia

Este proyecto es únicamente para fines académicos.
Podés modificarlo libremente para tus cursos o aprendizaje personal.