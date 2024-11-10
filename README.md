# Vector to Bevy

Supply a simple SVG file with a single path to get a `Vec<(x,y)>` of coordinates that correspond line. 

Although SVGs contain fill and stroke information, this project does not use any of that information. It simply extracts the lines from the path in the svg and converts it to points to use in your bevy project. 

## Image/Path Requirements

To ensure that the image matches the path, set the pixel/in (ppi) of the path to `96.0`. 

## Concept

Using svg files, create surfaces that span more than just rectangles.

## Usage

Create an svg image (perferrably a single path). Optionally, save a rasterized image file that will be mapped on top of the svg image. Example...

## Roadmap 
 
 Create the following for bevy-rapier2d:
- Paths are used as polyline mesh colliders. Align graphics (eg drawings) with the corresponding path `id`s. 
- Determine when colliders can intersect or collide with the player and object based on other metadata or conventions.

More to come...
