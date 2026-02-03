# Event Forms - Sistema de Formularios Multi-Paso con Cloudflare Workers

Una aplicación serverless completa para formularios multi-paso construida con Cloudflare Workers y Rust. Incluye matching inteligente de usuarios entre formularios, generación de códigos QR por paso, y un dashboard de administración completo.

## Características

### Formularios Multi-Paso
- Crea formularios con múltiples pasos, cada uno con su propia URL (`/f/{slug}/{step_number}`)
- Cada URL de paso puede convertirse en un código QR para fácil acceso
- Configura mensajes de completitud personalizados por paso y por formulario
- Soporte para imágenes en pantallas de completitud

### Tipos de Campo
- Texto, Email, Teléfono, Número
- Select dropdown, Radio buttons, Checkboxes
- Área de texto, Selector de fecha
- Campos ocultos

### Matching Inteligente de Usuarios
- **Sistema de Fingerprinting**: Configura qué campos identifican a un usuario (email, o combinaciones como nombre+colegio+edad)
- **Reconocimiento Cross-Form**: Cuando un usuario llena un formulario y luego visita otro, sus datos conocidos se pre-llenan
- **Gestión de Sesiones**: Seguimiento automático de sesiones basado en cookies
- **Datos Persistentes**: Configura qué campos persisten en las cookies del navegador

### Dashboard de Administración
- Protegido con autenticación básica
- CRUD completo para formularios, pasos y campos
- Analíticas en tiempo real:
  - Total de escaneos por código QR
  - Usuarios únicos vs recurrentes
  - Tasas de abandono del funnel por paso
  - Desglose por dispositivo (móvil/tablet/desktop)
  - Datos geográficos de los headers de Cloudflare
- Historial de interacciones por usuario
- Gestión de respuestas

### Stack Tecnológico
- **Backend**: Cloudflare Workers con Rust (WASM)
- **Base de Datos**: Cloudflare D1 (SQLite)
- **Frontend**: HTML/CSS/JS vanilla servido por el worker
- **Autenticación**: HTTP Basic Auth para rutas de admin

## Configuración

### Variables de Entorno

Configura estas en `wrangler.toml` o como secrets:

```toml
[vars]
ADMIN_USERNAME = "admin"
ADMIN_PASSWORD = "tu-password-seguro"
ENCRYPTION_KEY = "32-caracteres-de-clave-secreta!!"
COOKIE_DOMAIN = ""  # Dejar vacío para dominio actual
```

### Configuración de D1

1. Crear base de datos D1:
```bash
wrangler d1 create event-forms-db
```

2. Actualizar `wrangler.toml` con el ID de la base de datos:
```toml
[[d1_databases]]
binding = "DB"
database_name = "event-forms-db"
database_id = "tu-database-id"
```

3. Inicializar el schema llamando a la API:
```bash
curl -X POST https://tu-worker.workers.dev/api/init \
  -u admin:tu-password
```

## Rutas de la API

### Rutas Públicas
- `GET /f/:slug/:step` - Ver paso del formulario
- `POST /f/:slug/:step/submit` - Enviar paso del formulario
- `GET /f/:slug/complete` - Página de agradecimiento

### Rutas de Admin (requieren autenticación)
- `GET /admin` - Dashboard
- `GET /api/forms` - Listar formularios
- `POST /api/forms` - Crear formulario
- `GET /api/forms/:id` - Obtener formulario
- `PUT /api/forms/:id` - Actualizar formulario
- `DELETE /api/forms/:id` - Eliminar formulario
- `GET /api/forms/:id/stats` - Analíticas del formulario
- `GET /api/forms/:id/responses` - Respuestas del formulario
- `GET /api/forms/:form_id/steps` - Listar pasos
- `POST /api/forms/:form_id/steps` - Crear paso
- `GET /api/steps/:id/fields` - Listar campos
- `POST /api/steps/:id/fields` - Crear campo
- `GET /api/users` - Listar usuarios
- `GET /api/users/:id/history` - Historial de interacciones del usuario

## Configuración de Formularios

### Campos de Completitud de Sesión
Define qué campos deben llenarse para considerar una sesión "completa":
```json
["email"]
```
o
```json
["email", "phone"]
```

### Campos para Cookies
Campos que persisten en el navegador del usuario para pre-llenado:
```json
["email", "phone", "full_name"]
```

### Campos de Fingerprint
Combinaciones de campos que identifican únicamente a un usuario:
```json
[["email"], ["full_name", "school", "age"]]
```
Esto significa: matchear por email solo, O por la combinación de nombre+colegio+edad.

## Desarrollo

### Prerrequisitos
- Toolchain de Rust con target `wasm32-unknown-unknown`
- Wrangler CLI

### Build
```bash
cargo build --target wasm32-unknown-unknown
```

### Desarrollo Local
```bash
wrangler dev
```

### Desplegar
```bash
wrangler deploy
```

## Licencia

MIT
