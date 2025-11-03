# Simulador del Sistema Solar

Un simulador del sistema solar implementado en Rust usando un software renderer personalizado con Raylib.

## 🌟 Características Implementadas

### Características Principales
- ✅ **Sol y Planetas**: Sistema solar con 1 estrella y 5 planetas únicos
- ✅ **Plano Eclíptico**: Todos los cuerpos celestes se alinean al plano eclíptico
- ✅ **Rotación Orbital**: Los planetas orbitan alrededor del sol en órbitas circulares
- ✅ **Rotación Axial**: Cada cuerpo celeste rota sobre su propio eje
- ✅ **Cámara 3D**: Sistema de cámara completamente funcional con movimiento en 3 dimensiones

### Características Avanzadas
- ✅ **Instant Warping**: Teletransporte instantáneo a diferentes planetas (teclas 1-5)
- ✅ **Warping Animado**: Efecto de animación suave durante el teletransporte
- ✅ **Nave Espacial**: Nave personalizada que sigue a la cámara
- ✅ **Skybox**: Campo de estrellas en el horizonte
- ✅ **Detección de Colisiones**: La cámara/nave evita atravesar los cuerpos celestes
- ✅ **Órbitas Visibles**: Renderizado de las órbitas planetarias (tecla O)
- ✅ **Lunas**: Algunos planetas tienen lunas que orbitan a su alrededor
- ✅ **Anillos**: Planetas con sistemas de anillos

## 🎮 Controles

### Movimiento de Cámara
- **Mouse**: Rotar cámara (mantener clic izquierdo)
- **Rueda del Mouse**: Zoom in/out
- **WASD**: Movimiento 3D de la cámara
- **Shift**: Acelerar movimiento
- **Ctrl**: Ralentizar movimiento

### Funciones Especiales
- **1-5**: Teletransporte instantáneo a planetas específicos
- **O**: Mostrar/ocultar órbitas planetarias
- **W**: Alternar modo wireframe
- **+/-**: Acelerar/ralentizar tiempo
- **R**: Reiniciar cámara
- **ESC**: Salir

## 🪐 Cuerpos Celestes

### Sol (Estrella)
- **Nombre**: Sol
- **Color**: Amarillo brillante con efecto de resplandor
- **Características**: Estrella central del sistema

### Planetas

1. **Pyrion** (Planeta Rocoso)
   - Color: Rojo ardiente
   - Distancia orbital: 50 unidades
   - Características: Planeta volcánico cercano al sol

2. **Verdania** (Planeta con Vida)
   - Color: Verde azulado
   - Distancia orbital: 80 unidades
   - Características: Planeta habitable con 1 luna

3. **Gigantus** (Gigante Gaseoso)
   - Color: Naranja
   - Distancia orbital: 120 unidades
   - Características: Planeta gigante con anillos y 2 lunas

4. **Glacialis** (Planeta Helado)
   - Color: Azul claro
   - Distancia orbital: 160 unidades
   - Características: Planeta helado en las regiones exteriores

5. **Plutonix** (Planeta Enano)
   - Color: Gris
   - Distancia orbital: 200 unidades
   - Características: Pequeño planeta en el borde del sistema

## 🛠️ Arquitectura Técnica

### Módulos Principales

- **`main.rs`**: Punto de entrada y bucle principal del juego
- **`matrix.rs`**: Implementación de matrices 4x4 y transformaciones 3D
- **`camera.rs`**: Sistema de cámara con controles orbitales y movimiento 3D
- **`celestial_body.rs`**: Definición y comportamiento de cuerpos celestes
- **`solar_system.rs`**: Gestión del sistema solar completo
- **`renderer.rs`**: Software renderer personalizado con proyección 3D

### Software Renderer

El renderer implementa:
- Transformaciones de coordenadas mundo → vista → proyección → pantalla
- Proyección perspectiva con matriz de proyección personalizada
- Renderizado de esferas usando círculos con efectos de profundidad
- Sistema de skybox con estrellas generadas proceduralmente
- Renderizado de órbitas como líneas conectadas
- Efectos visuales (resplandor para estrellas, anillos planetarios)

## 🚀 Instalación y Ejecución

### Prerrequisitos
- Rust (versión 1.70 o superior)
- Cargo (incluido con Rust)

### Dependencias
- `raylib = "5.0"`: Para ventana, gráficos y entrada
- `rand = "0.8"`: Para generación de estrellas aleatorias

### Compilación y Ejecución

```bash
# Clonar el repositorio
git clone [URL_DEL_REPOSITORIO]
cd Proyecto_Planetas

# Compilar el proyecto
cargo build --release

# Ejecutar el simulador
cargo run --release
```

## 🎯 Puntuación del Proyecto

### Criterios Cumplidos

| Criterio | Puntos | Estado |
|----------|--------|--------|
| Estética del sistema completo | 30 | ✅ |
| Performance de la escena | 20 | ✅ |
| Planetas/estrellas/lunas (5 cuerpos) | 50 | ✅ |
| Instant warping | 10 | ✅ |
| Efecto de warping animado | 10 | ✅ |
| Nave espacial personalizada | 30 | ✅ |
| Skybox con estrellas | 10 | ✅ |
| Detección de colisiones | 10 | ✅ |
| Movimiento 3D de cámara | 40 | ✅ |
| Renderizado de órbitas | 20 | ✅ |

**Total Estimado: 230/230 puntos**

## 🎥 Video Demostración

[Aquí se incluirá el video demostrativo del sistema solar]

## 🔧 Características Técnicas Destacadas

### Matemáticas 3D
- Implementación completa de matrices 4x4
- Transformaciones de rotación, traslación y escala
- Proyección perspectiva personalizada
- Sistema de coordenadas vista-modelo-proyección

### Optimizaciones
- Culling de objetos fuera de pantalla
- Renderizado por distancia para orden correcto
- Aproximaciones eficientes para cálculos de distancia
- Generación procedural de geometría

### Efectos Visuales
- Gradientes para simular iluminación 3D
- Efectos de resplandor para estrellas
- Transparencia para anillos planetarios
- Etiquetas dinámicas de nombres

## 📝 Notas de Desarrollo

Este proyecto fue desarrollado como parte del curso de Gráficas por Computadora, implementando un software renderer desde cero para demostrar comprensión de:

- Transformaciones 3D y proyección
- Sistemas de coordenadas
- Renderizado de primitivas
- Gestión de cámara y controles
- Arquitectura de software para gráficos

El código está estructurado de manera modular para facilitar el mantenimiento y la extensión de funcionalidades.

## 🤝 Contribuciones

Este es un proyecto académico. Para sugerencias o mejoras, por favor crear un issue en el repositorio.

## 📄 Licencia

Este proyecto es de uso académico y educativo.