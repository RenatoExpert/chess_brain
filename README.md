# chess_brain
An Artificial Inteligence study for Chess

## Development Goals
- [ ] Functions to convert between position representations: 1d, cartesian 2d or human 2d
- [ ] Develop a basic CLI
- [ ] Be able to determine if a sequence is possible or not
- [ ] Be able to determine 'next possible moves' in a sequence
- [ ] Store match sequences (i am thinking about redis)
- [ ] Be able to determine statistical probability for each next movement
- [ ] Be able to choose next movement based on those statitics (with some entropy)

## Concepts
### Position representations
#### Index Integer
Is an 1D representation, chess tiles are numbered from 0 to 63.
#### Cartesian
A 2D representation using and (x,y) pair, each going from 0 to 7.
#### Human
The representation used on chess official rules, from A1 to H8.
#### Convertion
##### Pseudo code formula
```
to_ip (cp) -> u8 {
    return cp[x] + (cp[y]*8);
}
to_cp (ip) -> (u8, u8) {
    return { x: (ip % 8), y: (ip / 8) };
}
to_hp (cp) -> String {
    let letter: char = switch cp[0] {
        0: A
        1: B
        ...
    }
    let number = to_str (cp[1]+1);
    return letter+number;
}
```
##### Examples
- A1 = [0, 0] = 0
- C2 = [2, 1] = 10
- F7 = [6, 5] = 46
