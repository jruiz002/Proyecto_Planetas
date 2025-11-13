# Simulador del Sistema Solar 🌌

Un simulador del sistema solar implementado en Rust con software renderer personalizado.

## 🎥 Video Demostración

[![Simulador del Sistema Solar - Video Demostración](https://img.youtube.com/vi/iKaNzFiWErI/hqdefault.jpg)](https://youtu.be/iKaNzFiWErI)

*Haz clic en la imagen para ver el video completo - demostración de controles, warping y navegación 3D*

## 🌟 Características Implementadas

### Sistema Solar
- **1 Estrella**: Sol en el centro con efectos de resplandor
- **5 Planetas**: Pyrion, Verdania, Gigantus, Glacialis, Plutonix
- **3 Lunas**: Distribuidas entre los planetas
- **Anillos**: Sistema de anillos en Gigantus
- **Posiciones fijas**: Planetas estáticos para navegación consistente

### Funcionalidades Especiales
- ✅ **Instant Warping**: Teletransporte instantáneo (Shift+1-5)
- ✅ **Warping Animado**: Transición suave animada (1-5)
- ✅ **Skybox**: Campo de 1500 estrellas
- ✅ **Detección de Colisiones**: Previene atravesar planetas
- ✅ **Cámara 3D**: Movimiento libre en todas las direcciones
- ✅ **Órbitas Visibles**: Toggle para mostrar/ocultar órbitas

## 🎮 Controles

| Tecla | Función |
|-------|---------|
| **Mouse + Clic** | Rotar cámara |
| **Rueda Mouse** | Zoom in/out |
| **WASD** | Movimiento 3D |
| **Q/E** | Subir/bajar |
| **W** | Toggle modo warp |
| **1-5** | Warp animado a planetas |
| **Shift+1-5** | Warp instantáneo |
| **0** | Warp al Sol |
| **9** | Vista general del sistema |
| **O** | Toggle órbitas |

## 🛠️ Arquitectura del Proyecto

```
src/
├── main.rs           # Punto de entrada y bucle principal
├── camera.rs         # Sistema de cámara y warping
├── celestial_body.rs # Definición de planetas, lunas y sol
├── solar_system.rs   # Gestión del sistema completo
├── renderer.rs       # Software renderer 3D
└── matrix.rs         # Matemáticas 3D y transformaciones
```

### Módulos Principales

#### **main.rs**
- Inicialización de Raylib
- Bucle principal del juego
- Manejo de entrada de usuario
- Coordinación entre sistemas

#### **camera.rs**
- Sistema de cámara orbital
- Funciones de warp (animado e instantáneo)
- Detección y prevención de colisiones
- Controles de movimiento 3D

#### **celestial_body.rs**
- Estructura de datos para cuerpos celestes
- Propiedades: posición, radio, color, rotación
- Generación de geometría esférica
- Sistema de lunas y anillos

#### **solar_system.rs**
- Configuración del sistema solar
- Gestión de todos los cuerpos celestes
- Actualización y renderizado coordinado

#### **renderer.rs**
- Software renderer personalizado
- Pipeline 3D: mundo → vista → proyección → pantalla
- Renderizado de esferas, órbitas y skybox
- Efectos visuales y iluminación básica

#### **matrix.rs**
- Implementación de matrices 4x4
- Transformaciones 3D (rotación, traslación, escala)
- Proyección perspectiva
- Operaciones vectoriales

## 📦 Dependencias

### Raylib (v5.0)
```toml
raylib = "5.0"
```
**¿Por qué Raylib?**
- **Simplicidad**: API limpia para ventanas y entrada
- **Performance**: Renderizado eficiente en GPU
- **Cross-platform**: Funciona en Windows, macOS, Linux
- **Rust bindings**: Integración nativa con Rust

### Rand (v0.8)
```toml
rand = "0.8"
```
**¿Por qué Rand?**
- **Skybox**: Generación procedural de estrellas
- **Variabilidad**: Posiciones aleatorias para efectos visuales
- **Estándar**: Biblioteca estándar para números aleatorios en Rust

## 🚀 Instalación y Ejecución

### Prerrequisitos
- [Rust](https://rustup.rs/) (versión 1.70+)
- Git

### Pasos
```bash
# Clonar el repositorio
git clone https://github.com/jruiz002/Proyecto_Planetas.git
cd Proyecto_Planetas

# Compilar y ejecutar
cargo run --release
```

## 🔧 Características Técnicas

### Software Renderer
- **Pipeline 3D completo**: Transformaciones mundo-vista-proyección
- **Proyección perspectiva**: Matrices 4x4 personalizadas
- **Renderizado por profundidad**: Ordenamiento correcto de objetos
- **Optimizaciones**: Culling y nivel de detalle dinámico

### Sistema de Warping
- **Interpolación suave**: Función `smooth_step` para animaciones
- **Múltiples destinos**: Sol, planetas individuales, vista general
- **Navegación intuitiva**: Warp instantáneo o animado según preferencia

### Detección de Colisiones
- **Buffer de seguridad**: 1.6x radio de cada cuerpo celeste
- **Reposicionamiento automático**: Empuje suave fuera de la zona de colisión
- **Feedback visual**: Indicador en UI cuando está activo

---

*Proyecto desarrollado para el curso de Gráficas por Computadora - Universidad del Valle de Guatemala*
