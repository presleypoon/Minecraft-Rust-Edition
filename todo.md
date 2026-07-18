# Todo List

## Cave Game Tech Test

### Blocks

[x] Air

[x] Grass

[x] Rock

### Non-mob entities

#### Player

[ ] Add collusion

[ ] Has a height of 17 blocks

[x] Move
  
### World generation

#### Caves

[ ] Pass from the uppermost layer of the grass block to the penultimate layer of stone

#### Chunks

[x] The size of each chunk are 16x16

[x] The player spawns in a 256×64×256 tile map

[x] It was possible to fall out of the world, but it would not kill the player

### Gameplay

#### Controls

[ ] WASD key use for movement

[ ] Space key for jumping

[ ] Holding down R causes the player to respawn in a random X and Z coordinate at Y=74 every game tick

[ ] If the player falls into the void, they fall indefinitely (in this, it will be until they hit the i32 limit) until they press R

### General

#### Light

[ ] The lighting engine in Classic and pre-Classic was simple, with only 2 light levels, bright and dark

[ ] "Sunlight" is emitted by the top edge of the map and hits any block that is under it, regardless of distance It passes through transparent locks to light blocks underneath

[ ] Blocks that do not receive light are in a dim shadow that remains at the same level of brightness no matter how far they are from a light source

[ ] Blocks which are darkened also have a layer of thick, black fog applied to them, appearing darker when looked at from further distances This caused weird visual quirks
