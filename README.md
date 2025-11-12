# Proyecto 3: Space Travel

Un simulador 3D del sistema solar que te permite explorar los planetas desde una nave espacial, con efectos visuales avanzados y controles intuitivos.

## Características Principales

- **5 Planetas Detallados**: Explora el Sol, Mercurio, Tierra, Marte y Urano, cada uno con texturas y características únicas.
- **Warping Instantáneo**: Viaja rápidamente entre planetas usando las teclas numéricas (1-5).
- **Órbitas 3D**: Visualización precisa de las órbitas planetarias en el espacio 3D.
- **Sistema de Colisiones**: La cámara evita atravesar los planetas, manteniendo una distancia segura.
- **Iluminación y Sombras**: Efectos de iluminación realistas que mejoran la inmersión.
- **Cámara en Tercera Persona**: Vista desde la nave espacial con controles intuitivos.

## Video del funcionamiento

[![Ver en YouTube](https://img.youtube.com/vi/J4IJ4imB9f8/0.jpg)](https://youtu.be/J4IJ4imB9f8)

## Controles de Cámara

### Rotación
- **W**: Rotar hacia arriba (aumentar pitch)
- **S**: Rotar hacia abajo (disminuir pitch)
- **A**: Rotar a la izquierda (aumentar yaw)
- **D**: Rotar a la derecha (disminuir yaw)

### Zoom
- **Flecha Arriba (↑)**: Acercar
- **Flecha Abajo (↓)**: Alejar

### Desplazamiento (Pan)
- **Flecha Izquierda (←) / Q**: Mover a la izquierda
- **Flecha Derecha (→) / E**: Mover a la derecha
- **R**: Mover hacia arriba
- **F**: Mover hacia abajo

### Warping entre Planetas
- **1**: Vista completa del sistema solar
- **2**: Vista desde arriba del sistema solar
- **3**: Warp a la Tierra
- **4**: Warp a Marte
- **5**: Warp a Urano


## Instalación

1. Asegúrate de tener instalado [Rust](https://www.rust-lang.org/tools/install) en tu sistema.
2. Clona este repositorio:
   ```bash
   git clone https://github.com/Branuvg/blender_graficas.git
   cd blender_graficas/ship
   ```
3. Ejecuta el proyecto:
   ```bash
   cargo run --release
   ```

## Detalles Técnicos

### Prevención de Colisiones
El sistema implementa un algoritmo de prevención de colisiones que:
- Detecta cuando la cámara se acerca demasiado a un planeta
- Ajusta automáticamente la posición de la cámara para mantener una distancia segura
- Suaviza el movimiento para evitar cambios bruscos en la cámara

### Sistema de Órbitas
- Las órbitas son visualizadas como círculos en 3D
- Cada planeta sigue su propia órbita alrededor del Sol
- Las órbitas son precisas y se basan en parámetros astronómicos reales

### Shaders Personalizados
Cada planeta y la nave espacial tienen shaders personalizados que incluyen:
- Mapeo de texturas
- Efectos de iluminación
- Reflexiones especulares
- Efectos atmosféricos (en planetas con atmósfera)

## Estructura del Proyecto

```
ship/
├── src/
│   ├── main.rs          # Punto de entrada principal
│   ├── camera.rs        # Lógica de la cámara y controles
│   ├── shaders.rs       # Shaders personalizados
│   ├── obj.rs           # Carga de modelos 3D
│   ├── matrix.rs        # Operaciones matriciales
│   ├── fragment.rs      # Procesamiento de fragmentos
│   ├── vertex.rs        # Procesamiento de vértices
│   ├── triangle.rs      # Rasterización de triángulos
│   └── framebuffer.rs   # Manejo del búfer de fotogramas
├── assets/             # Recursos del juego
├── models/             # Modelos 3D
└── Cargo.toml          # Configuración del proyecto
```

## Créditos

- Desarrollado como proyecto de gráficos por computadora
- Utiliza la biblioteca Raylib para renderizado
- Modelos 3D creados específicamente para este proyecto
