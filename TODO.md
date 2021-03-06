Engine
======

Short Term
----------

- add grass and trees
  - done: palm trees on sand
  - done: grass as decoration
  - done: make grass thin rather than tile-shaped
  - todo: apple trees onto grass
  - todo: pine trees onto mountains
  - todo: snow
- clouds in sky
  - smaller and larger clouds
  - make look better with different orientations
- routines for mouse click to tile
  - first without elevation, then figure out elevation
  - and then figure out decorations
- make map_x, map_y use Point for consistency
- fix rotation when zoomed in (stay centered on same tile)
- error handling if texture type not found
- break from ideas and do some cleanup and *gasp* commenting (use ///)
- make draw_offset_[xy] optional in spritesheet.toml
- make struct holding information like orientation and stuff
  rather than dumping in an unattached variable in the event
  loop
- add slight border to one side and top of tiles so they don't visually
    merge together when they shouldn't
- set an icon

Medium Term
-----------
- better data structure for hex stacks, 'cause the current thing is
   unreadably ugly
  - investigate not using hashes, or using FnvHashMap
- make the stuff in interface into a gameloop struct
- rework pan/scroll key handling to allow more combos
- move splash screen to its own function
 - make customizable
- more mouse cursors
 - "control" -- compass, menus, minimap
 - edge scroll
 - interaction-available (hover over active)
 - drag/carry
 - make scale with window size?
- dynamic decorations:
 - spreading grass
 - growing trees
 - coconuts, apples, pine cones (see below for more)
- terrain water -- rivers (possibly also lakes, for larger maps)
- clay next to the rivers
- lava? obsidian? (obsidian is good for sharp tools?)
- possibly separate sides and top of hexes, so they can more easily be
  variable height -- performance optimization
- move cloud data types out of weather controller -- that way,
  both weather controller and interface include cloud, but interface
  does not have to include the controller
- make wind direction variable, and allow clouds to go "behind the horizon"
- make a game setup function rather than dumping that at the beginning
    of the event loop
- architectural decision re universal event loop with different states
  _or_ different loops for game menu and game itself
- moving things: actors!
  - turtles seem like a nice place to start.
    They're hexagons, after all.
  - movement in one thread
  - decision making in its own thread?
- game menu
  - exit
  - save/load game
  - game speed
- describe UI items (compass, menu controls) in toml, including
  location on screen
- Optimize map redraw so animations don't require a full-universe
  redraw. ("Dirty columns?")
- Add objects (start with turtle eggs?)
- proper error handling for all of those unwrap()s

Longer Term
-----------
- different sprites for each scale level for prettiness
- object-interaction menus (decorations, actors, objects)
- possible terrain-info window (useful for civ or MoM-type games)
- split engine from game; game starts taking shape as a _game_
- more creatures
  - monkeys
  - alpacas 
  - jaguars?
  - fish / dolphins / sharks
  - bees! (need flowers. beehives, then, of course)
- the actual human characters for the game
- map generator: build up layers of stone and dirt or sand
- buildings. think I'm going to go for hex-sized huts rather than
   wall-building. Maybe leave room for both?
- minimap?
- animate zoom
- animate rotation (hard because it needs to be in 3d... or at least
   pseudo-3d)
- when zooming in with scroll wheel, center on mouse
- dynamic terrain
  - "mine" or build tiles
  - ocean water levels -- tides!
    - clear water?
  - spreading rivers
- make apples, coconuts, and pine cones spread to new trees
- time passing
  - night day
  - sun, moon in sky
- make clouds go in different directions
- weather -- different kinds of clouds
- separate UI sprites from game-world sprites
- selectable window sizes other than 960x540
- save window size/position when going from windowed to fullscreen; restore if needed
- optimization: dirty rects for clouds rather than repainting whole sky

Far Out but Planned
-------------------
- go from island to generic map stuff
- make the sky optional for non-island maps
- change generation routine to an embedded script. (rhai? rlua?)
  - un-hardcode terrain types
- handle multiple aspect ratios correctly
- move island generation and the specific island terrain types
   to an add-on
- target games:
  - island survival (rimworld + sims + minecraft)
  - ai evolution simulation (plant / herbivore / carnivore)
  - civ-like city builder
  - roguelike generative hack-and-slash RPG
- shadows
- optimization: dynamic map drawing instead of onto one 
   big texture. I know how to do this from the 1990s. :)
- add flat-top hex orientation (and six more directions)
- transition hexes (from one terrain type to another)
- sound effects
- music

Game
====
- Still working on the engine. :)

