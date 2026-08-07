# Wallhaven API v1 - Documentación

> **URL Base:** `https://wallhaven.cc/api/v1`  
> **Documentación oficial:** [wallhaven.cc/help/api](https://wallhaven.cc/help/api)

---

## Tabla de Contenidos

- [Autenticación](#autenticación)
- [Rate Limiting](#rate-limiting)
- [Endpoints](#endpoints)
  - [Obtener información de un wallpaper](#obtener-información-de-un-wallpaper)
  - [Buscar wallpapers](#buscar-wallpapers)
  - [Obtener información de un tag](#obtener-información-de-un-tag)
  - [Obtener configuración del usuario](#obtener-configuración-del-usuario)
  - [Obtener colecciones de un usuario](#obtener-colecciones-de-un-usuario)
  - [Obtener wallpapers de una colección](#obtener-wallpapers-de-una-colección)
- [Parámetros de búsqueda](#parámetros-de-búsqueda)
- [Códigos de error](#códigos-de-error)
- [Ejemplos de uso](#ejemplos-de-uso)

---

## Autenticación

La API de Wallhaven no requiere autenticación para la mayoría de los endpoints públicos. Sin embargo, algunas funcionalidades requieren una **API Key**:

- Acceder a wallpapers **NSFW**
- Ver colecciones **privadas**
- Obtener **configuración del usuario**
- Realizar búsquedas con los **ajustes de navegación del usuario**

### Obtener tu API Key

1. Crea una cuenta en [wallhaven.cc](https://wallhaven.cc)
2. Ve a tu perfil → **Settings** → **Account**
3. Genera tu API Key

### Uso de la API Key

Pasa la API Key como parámetro de consulta:

```
https://wallhaven.cc/api/v1/w/<ID>?apikey=<TU_API_KEY>
```

---

## Rate Limiting

> **Límite:** 45 peticiones por minuto  
> Si superas este límite, recibirás un error **429 - Too many requests**.

---

## Endpoints

### Obtener información de un wallpaper

```
GET /w/{id}
```

Obtiene información detallada sobre un wallpaper específico.

**Parámetros:**

| Parámetro | Tipo   | Requerido | Descripción                    |
|-----------|--------|-----------|--------------------------------|
| `id`      | string | Sí        | ID del wallpaper               |
| `apikey`  | string | No*       | API Key (requerido para NSFW)  |

\* Requerido para acceder a wallpapers NSFW.

**Ejemplo de respuesta:**

```json
{
  "data": {
    "id": "94x38z",
    "url": "https://wallhaven.cc/w/94x38z",
    "short_url": "http://whvn.cc/94x38z",
    "uploader": {
      "username": "test-user",
      "group": "User",
      "avatar": {
        "200px": "https://wallhaven.cc/images/user/avatar/200/11_3339efb2a813.png",
        "128px": "https://wallhaven.cc/images/user/avatar/128/11_3339efb2a813.png",
        "32px": "https://wallhaven.cc/images/user/avatar/32/11_3339efb2a813.png",
        "20px": "https://wallhaven.cc/images/user/avatar/20/11_3339efb2a813.png"
      }
    },
    "views": 12,
    "favorites": 0,
    "source": "",
    "purity": "sfw",
    "category": "anime",
    "dimension_x": 6742,
    "dimension_y": 3534,
    "resolution": "6742x3534",
    "ratio": "1.91",
    "file_size": 5070446,
    "file_type": "image/jpeg",
    "created_at": "2018-10-31 01:23:10",
    "colors": [
      "#000000",
      "#abbcda",
      "#424153",
      "#66cccc",
      "#333399"
    ],
    "path": "https://w.wallhaven.cc/full/94/wallhaven-94x38z.jpg",
    "thumbs": {
      "large": "https://th.wallhaven.cc/lg/94/94x38z.jpg",
      "original": "https://th.wallhaven.cc/orig/94/94x38z.jpg",
      "small": "https://th.wallhaven.cc/small/94/94x38z.jpg"
    },
    "tags": [
      {
        "id": 1,
        "name": "anime",
        "alias": "Chinese cartoons",
        "category_id": 1,
        "category": "Anime & Manga",
        "purity": "sfw",
        "created_at": "2015-01-16 02:06:45"
      }
    ]
  }
}
```

---

### Buscar wallpapers

```
GET /search
```

Busca wallpapers según criterios específicos.

**Parámetros de consulta:**

| Parámetro    | Tipo   | Requerido | Descripción                                      | Valores posibles / Ejemplos |
|--------------|--------|-----------|--------------------------------------------------|-----------------------------|
| `q`          | string | No        | Término de búsqueda                              | `nature`, `anime`, etc.     |
| `categories` | string | No        | Categorías (general/anime/people)                | `100`, `110`, `111`         |
| `purity`     | string | No        | Nivel de pureza (sfw/sketchy/nsfw)               | `100`, `110`, `111`         |
| `sorting`    | string | No        | Método de ordenamiento                           | `date_added`, `relevance`, `random`, `views`, `favorites`, `toplist` |
| `order`      | string | No        | Orden ascendente/descendente                     | `desc`, `asc`               |
| `topRange`   | string | No        | Rango de tiempo para toplist                     | `1d`, `3d`, `1w`, `1M`, `3M`, `6M`, `1y` |
| `atleast`    | string | No        | Resolución mínima                                | `1920x1080`                 |
| `resolutions`| string | No        | Resoluciones exactas                             | `1920x1080,1920x1200`       |
| `ratios`     | string | No        | Relaciones de aspecto                            | `16x9,16x10`                |
| `colors`     | string | No        | Color de búsqueda                                | Ver lista de colores abajo  |
| `page`       | int    | No        | Número de página                                 | `1`, `2`, `3`...            |
| `seed`       | string | No        | Semilla para resultados aleatorios               | `abc123`                    |
| `apikey`     | string | No        | API Key                                          | Tu API Key                  |

> **Nota:** Si proporcionas un `apikey`, la búsqueda se realizará con los ajustes de navegación de ese usuario.

**Ejemplo de respuesta:**

```json
{
  "data": [
    {
      "id": "94x38z",
      "url": "https://wallhaven.cc/w/94x38z",
      "short_url": "http://whvn.cc/94x38z",
      "views": 6,
      "favorites": 0,
      "source": "",
      "purity": "sfw",
      "category": "anime",
      "dimension_x": 6742,
      "dimension_y": 3534,
      "resolution": "6742x3534",
      "ratio": "1.91",
      "file_size": 5070446,
      "file_type": "image/jpeg",
      "created_at": "2018-10-31 01:23:10",
      "colors": ["#000000", "#abbcda", "#424153", "#66cccc", "#333399"],
      "path": "https://w.wallhaven.cc/94/wallhaven-94x38z.jpg",
      "thumbs": {
        "large": "https://th.wallhaven.cc/lg/94/94x38z.jpg",
        "original": "https://th.wallhaven.cc/orig/94/94x38z.jpg",
        "small": "https://th.wallhaven.cc/small/94/94x38z.jpg"
      }
    }
  ],
  "meta": {
    "current_page": 1,
    "last_page": 36,
    "per_page": 24,
    "total": 848,
    "query": "test",
    "seed": "abc123"
  }
}
```

---

### Obtener información de un tag

```
GET /tag/{id}
```

Obtiene información sobre un tag específico.

**Parámetros:**

| Parámetro | Tipo   | Requerido | Descripción      |
|-----------|--------|-----------|------------------|
| `id`      | int    | Sí        | ID del tag       |

**Ejemplo de respuesta:**

```json
{
  "data": {
    "id": 1,
    "name": "anime",
    "alias": "Chinese cartoons",
    "category_id": 1,
    "category": "Anime & Manga",
    "purity": "sfw",
    "created_at": "2015-01-16 02:06:45"
  }
}
```

---

### Obtener configuración del usuario

```
GET /settings?apikey={apikey}
```

Obtiene los ajustes de navegación del usuario autenticado.

> **Requiere API Key.**

**Ejemplo de respuesta:**

```json
{
  "data": {
    "thumb_size": "orig",
    "per_page": "24",
    "purity": ["sfw", "sketchy", "nsfw"],
    "categories": ["general", "anime", "people"],
    "resolutions": ["1920x1080", "2560x1440"],
    "aspect_ratios": ["16x9"],
    "toplist_range": "6M",
    "tag_blacklist": ["blacklist tag", "another"],
    "user_blacklist": [""]
  }
}
```

---

### Obtener colecciones de un usuario

```
GET /collections/{username}
GET /collections?apikey={apikey}
```

Lista las colecciones de un usuario. La segunda forma lista las colecciones del usuario autenticado (incluyendo privadas).

**Parámetros:**

| Parámetro  | Tipo   | Requerido | Descripción                |
|------------|--------|-----------|----------------------------|
| `username` | string | Sí*       | Nombre de usuario          |
| `apikey`   | string | No**      | API Key                    |

\* Requerido para listar colecciones de otro usuario.  
\*\* Requerido para listar tus propias colecciones (incluidas las privadas).

**Ejemplo de respuesta:**

```json
{
  "data": [
    {
      "id": 15,
      "label": "Default",
      "views": 38,
      "public": 1,
      "count": 10
    },
    {
      "id": 17,
      "label": "This is another collection",
      "views": 6,
      "public": 1,
      "count": 7
    }
  ]
}
```

---

### Obtener wallpapers de una colección

```
GET /collections/{username}/{collection_id}
```

Obtiene el listado de wallpapers dentro de una colección.

**Parámetros:**

| Parámetro       | Tipo   | Requerido | Descripción                |
|-----------------|--------|-----------|----------------------------|
| `username`      | string | Sí        | Nombre de usuario          |
| `collection_id` | int    | Sí        | ID de la colección         |
| `apikey`        | string | No*       | API Key                    |
| `page`          | int    | No        | Número de página           |
| `purity`        | string | No        | Filtro de pureza           |

\* Requerido para acceder a colecciones privadas.

> **Nota:** El resultado es similar al de búsqueda, pero solo el filtro `purity` está disponible.

---

## Parámetros de búsqueda

### Categorías (`categories`)

Formato: tres dígitos binarios `G A P` (General, Anime, People)

| Valor | General | Anime | People |
|-------|---------|-------|--------|
| `100` | ✓       | ✗     | ✗      |
| `110` | ✓       | ✓     | ✗      |
| `111` | ✓       | ✓     | ✓      |
| `010` | ✗       | ✓     | ✗      |
| `011` | ✗       | ✓     | ✓      |
| `001` | ✗       | ✗     | ✓      |

### Pureza (`purity`)

Formato: tres dígitos binarios `S K N` (SFW, Sketchy, NSFW)

| Valor | SFW | Sketchy | NSFW |
|-------|-----|---------|------|
| `100` | ✓   | ✗       | ✗    |
| `110` | ✓   | ✓       | ✗    |
| `111` | ✓   | ✓       | ✓    |

> **NSFW requiere API Key válida.**

### Ordenamiento (`sorting`)

| Valor       | Descripción                          |
|-------------|--------------------------------------|
| `date_added`| Fecha de subida (por defecto)        |
| `relevance` | Relevancia                           |
| `random`    | Aleatorio (usa `seed` para páginas)  |
| `views`     | Número de vistas                     |
| `favorites` | Número de favoritos                  |
| `toplist`   | Top list (requiere `topRange`)       |

### Rango de toplist (`topRange`)

| Valor | Descripción       |
|-------|-------------------|
| `1d`  | Último día        |
| `3d`  | Últimos 3 días    |
| `1w`  | Última semana     |
| `1M`  | Último mes        |
| `3M`  | Últimos 3 meses   |
| `6M`  | Últimos 6 meses   |
| `1y`  | Último año        |

### Colores (`colors`)

| Color   | Código  | Color   | Código  | Color   | Código  |
|---------|---------|---------|---------|---------|---------|
| Rojo    | `660000`| Rosa    | `ea4c88`| Verde   | `77cc33`|
| Rojo    | `990000`| Púrpura | `993399`| Verde   | `669900`|
| Rojo    | `cc0000`| Púrpura | `663399`| Verde   | `336600`|
| Rojo    | `cc3333`| Azul    | `333399`| Amarillo| `666600`|
| Rosa    | `cc6699`| Azul    | `0066cc`| Amarillo| `999900`|
| Rosa    | `ff99cc`| Azul    | `0099cc`| Amarillo| `cccc33`|
| Naranja | `ff6600`| Cyan    | `66cccc`| Amarillo| `ffff00`|
| Naranja | `ff9900`| Cyan    | `77cccc`| Amarillo| `ffcc33`|
| Naranja | `ffcc33`| Verde   | `99cc33`| Naranja | `ff9900`|
| Marrón  | `cc6633`| Verde   | `66cc99`| Blanco  | `ffffff`|
| Marrón  | `996633`| Verde   | `33cc99`| Gris    | `999999`|
| Marrón  | `663300`| Verde   | `33cccc`| Gris    | `cccccc`|
| Negro   | `000000`|         |         | Gris    | `424153`|

### Sintaxis de búsqueda avanzada (`q`)

| Sintaxis              | Descripción                              |
|-----------------------|------------------------------------------|
| `tagname`             | Búsqueda difusa por tag/palabra clave    |
| `-tagname`            | Excluir un tag/palabra clave             |
| `+tag1 +tag2`         | Debe tener tag1 Y tag2                   |
| `+tag1 -tag2`         | Debe tener tag1 y NO tag2                |
| `@username`           | Subidas de un usuario específico         |
| `id:123`              | Búsqueda exacta por ID de tag            |
| `type:png` / `type:jpg` | Buscar por tipo de archivo               |
| `like:wallpaper_id`   | Encontrar wallpapers con tags similares  |

---

## Códigos de error

| Código | Descripción            | Causa común                                      |
|--------|------------------------|--------------------------------------------------|
| `401`  | Unauthorized           | API Key inválida o faltante para contenido NSFW  |
| `429`  | Too many requests      | Se superó el límite de 45 peticiones/minuto      |

---

## Ejemplos de uso

### JavaScript (Fetch)

```javascript
// Buscar wallpapers SFW de naturaleza
fetch('https://wallhaven.cc/api/v1/search?q=nature&categories=111&purity=100&sorting=toplist&topRange=1M')
  .then(response => response.json())
  .then(data => console.log(data));

// Obtener detalle de un wallpaper
fetch('https://wallhaven.cc/api/v1/w/94x38z')
  .then(response => response.json())
  .then(data => console.log(data));

// Búsqueda con API Key
fetch('https://wallhaven.cc/api/v1/search?q=anime&apikey=TU_API_KEY')
  .then(response => response.json())
  .then(data => console.log(data));
```

### cURL

```bash
# Búsqueda básica
curl "https://wallhaven.cc/api/v1/search?q=mountains&categories=111&purity=100"

# Obtener detalle de wallpaper
curl "https://wallhaven.cc/api/v1/w/94x38z"

# Con API Key
curl "https://wallhaven.cc/api/v1/search?apikey=TU_API_KEY&q=anime"
```

---

## Notas importantes

- **No ejecutes scripts de descarga masiva.** Wallhaven es un proyecto pequeño sin publicidad; las descargas masivas pueden afectar su infraestructura.
- Los resultados de búsqueda están limitados a **24 wallpapers por página** por defecto. Con API Key, puedes cambiar esto a 24, 32 o 64 en tus ajustes de usuario.
- Al usar `sorting=random`, se genera una `seed` que puedes pasar entre páginas para evitar repeticiones.
- Las búsquedas exactas por tag (`id:##`) incluyen el nombre del tag resuelto en los metadatos de la respuesta.

---

*Documentación generada a partir de la API oficial de Wallhaven v1.*
