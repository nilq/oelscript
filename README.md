# ØlScript

We all know TypeScript. This is ØlScript.

## Example

```
// list of øl
øl øller = ["tuborg", "stick a finger in the soil"]
øl player = {
  x: 100
  y: 100
}

øl move(player, dx, dy) =
  player.x = player.x + dx
  player.y = player.y + dy

øl check(x) =
  øl x < 10:
    øl "x is less"
  ølse:
    øl "x is greater or equal to"
    
check(4)
move(player, 10, -10)
```
