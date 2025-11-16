# Simulador del Sistema Solar 🌌

Proyecto de renderizado 3D en tiempo real utilizando un **Software Rasterizer** completamente personalizado en Rust. El sistema implementa un pipeline gráfico modular desde cero, sin uso de GPU, renderizando un sistema solar interactivo con planetas, lunas, anillos y efectos visuales.


- ✅ **Framebuffer personalizado**: Sistema de buffer de píxeles propio (960,000 píxeles)
- ✅ **Pipeline en etapas separadas**: 4 módulos independientes (vertex → assembly → rasterizer → fragment)
- ✅ **Modelo sphere.obj**: 482 vértices importados desde archivo Wavefront OBJ
- ✅ **Transformaciones con matrices 4x4**: Model → View → Projection → Viewport

---

## 🌟 Características del Sistema

### Sistema Solar
- **1 Estrella**: Sol central con emisión de luz
- **5 Planetas**: Pyrion, Verdania, Gigantus, Glacialis, Plutonix
- **3 Lunas**: Distribuidas entre planetas
- **Sistema de anillos**: Gigantus con anillos procedurales
- **Órbitas visuales**: Trazado de caminos orbitales

### Efectos Visuales
- **Phong Shading**: Iluminación difusa y ambiente
- **Sistema de LOD**: 4 niveles de detalle dinámicos (1x, 2x, 4x, 8x skip)
- **Skybox procedural**: 1500 estrellas generadas aleatoriamente
- **Backface Culling**: Optimización de triángulos no visibles
- **Interpolación baricéntrica**: Colores y normales suaves

### Optimizaciones de Performance
- Compilación en modo `--release`
- Level of Detail (LOD) basado en distancia
- Culling de triángulos traseros
- Salto de píxeles en renderizado lejano
- Renderizado incremental de estrellas

---

## 🎮 Controles

| Tecla | Función |
|-------|---------|
| **Mouse** | Rotar cámara orbital |
| **Rueda Mouse** | Zoom in/out |
| **W/A/S/D** | Movimiento horizontal |
| **Q/E** | Subir/bajar |
| **1-5** | Warp animado a planetas |
| **Shift+1-5** | Warp instantáneo |
| **O** | Toggle órbitas |

---

## 🏗️ Arquitectura del Proyecto

### Estructura de Módulos
```
src/
├── main.rs                   # Game loop y UI
├── renderer.rs               # Orquestador del pipeline
├── framebuffer.rs            # Buffer de píxeles personalizado
├── vertex_shader.rs          # Etapa 1: Transformaciones de vértices
├── primitive_assembly.rs     # Etapa 2: Ensamblado de triángulos
├── rasterizer.rs             # Etapa 3: Conversión a fragmentos
├── fragment_shader.rs        # Etapa 4: Cálculo de colores finales
├── camera.rs                 # Sistema de cámara 3D
├── matrix.rs                 # Operaciones con matrices 4x4
├── obj_loader.rs             # Importador de archivos Wavefront OBJ
├── celestial_body.rs         # Estructuras de planetas/lunas
└── solar_system.rs           # Configuración del sistema solar
```

### Descripción de Módulos

**Pipeline Gráfico (4 Etapas):**

1. **vertex_shader.rs**: Transforma vértices del modelo local a coordenadas de pantalla
   - Aplica matrices: Model → View → Projection → Viewport
   - Calcula posición final y normales transformadas

2. **primitive_assembly.rs**: Ensambla triángulos a partir de vértices
   - Lee índices de caras del modelo OBJ
   - Implementa backface culling (descarta triángulos traseros)
   - Realiza frustum culling

3. **rasterizer.rs**: Convierte triángulos en fragmentos (píxeles)
   - Calcula coordenadas baricéntricas para interpolación
   - Sistema LOD con 4 niveles de detalle
   - Genera fragmentos con atributos interpolados (color, normal, profundidad)

4. **fragment_shader.rs**: Calcula color final de cada píxel
   - Iluminación Phong (ambiente + difusa)
   - Modo emisivo para objetos brillantes (Sol)
   - Aplicación de colores base

**Sistema de Renderizado:**

- **renderer.rs**: Orquestador que conecta las 4 etapas del pipeline
- **framebuffer.rs**: Buffer personalizado de 960,000 píxeles (1200x800)
- **obj_loader.rs**: Carga sphere.obj (482 vértices, 960 caras)

**Sistemas de Soporte:**

- **camera.rs**: Cámara 3D con detección de colisiones
- **matrix.rs**: Operaciones con matrices 4x4 (multiplicación, inversión, transformaciones)
- **celestial_body.rs**: Propiedades de planetas (posición, radio, color, órbita)
- **solar_system.rs**: Configuración de todos los cuerpos celestes

---

## 🔄 Flujo del Pipeline Gráfico

```
Modelo OBJ (sphere.obj)
        ↓
┌───────────────────────┐
│  1. VERTEX SHADER     │
│  - Model Matrix       │
│  - View Matrix        │
│  - Projection Matrix  │
│  - Viewport Matrix    │
└───────────────────────┘
        ↓
┌───────────────────────┐
│  2. PRIMITIVE         │
│     ASSEMBLY          │
│  - Ensamblar triángulos│
│  - Backface culling   │
│  - Frustum culling    │
└───────────────────────┘
        ↓
┌───────────────────────┐
│  3. RASTERIZER        │
│  - Coordenadas        │
│    baricéntricas      │
│  - Sistema LOD        │
│  - Generar fragmentos │
└───────────────────────┘
        ↓
┌───────────────────────┐
│  4. FRAGMENT SHADER   │
│  - Phong lighting     │
│  - Color final        │
│  - Emisión (Sol)      │
└───────────────────────┘
        ↓
   Framebuffer (960,000 píxeles)
        ↓
   Pantalla (Raylib Texture)
```

---

## 🚀 Ejecución

Para levantar el proyecto:

```bash
cargo run --release
```

---

*Proyecto desarrollado para el curso de Gráficas por Computadora - Universidad del Valle de Guatemala*
