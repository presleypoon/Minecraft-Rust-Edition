# Todo List

## Cave Game Tech Test

### Blocks

- [x] Air
- [x] Grass
- [x] Rock

### Non-mob entities

#### Player

1. [ ] Add collusion
2. [ ] Has a height of 1.7 blocks.
3. [x] Move
  
### World generation

#### Caves

1. [ ] Pass from the uppermost layer of the grass block to the penultimate layer of stone.

#### Chunks

1. [x] The size of each chunk are 16x16.
2. [x] The player spawns in a 256×64×256 tile map.
3. [x] It was possible to fall out of the world, but it would not kill the player.

### Gameplay

#### Controls

1. [ ] WASD key use for movement.
2. [ ] Space key for jumping.
3. [ ] Holding down R causes the player to respawn in a random X and Z coordinate at Y=74 every game tick
4. [ ] If the player falls into the void, they fall indefinitely (in this, it will be until they hit the i32 limit) until they press R.

### General

#### Light

1. [ ] The lighting engine in Classic and pre-Classic was simple, with only 2 light levels, bright and dark.
2. [ ] "Sunlight" is emitted by the top edge of the map and hits any block that is under it, regardless of distance. It passes through transparent locks to light blocks underneath.
3. [ ] Blocks that do not receive light are in a dim shadow that remains at the same level of brightness no matter how far they are from a light ource.
4. [ ] Blocks which are darkened also have a layer of thick, black fog applied to them, appearing darker when looked at from further distances. This caused weird visual quirks.
