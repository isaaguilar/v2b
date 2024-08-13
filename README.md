# Vector to Bevy

Supply a simple SVG file with a single path to get a `Vec<(x,y)>` of coordinates that correspond line.

## Image/Path Requirements

To ensure that the image matches the path, set the pixel/in (ppi) or the path to `96.0`. 

## Concept

Using svg files, create surfaces that span more than just rectangles.

## Roadmap 
 
 Create the following for bevy-rapier2d:
- Paths are used as polyline mesh colliders. Align graphics (eg drawings) with the corresponding path `id`s. 
- Determine when colliders can intersect or collide with the player and object based on other metadata or conventions.

More to come...
