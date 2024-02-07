# ele-forge

```
 $ cargo run -- Umbrella 1

The cost of Umbrella (1) is 850000, or buy it for None
Required elements (1):
--> Element: Plastic, Cost: 340000, Forge: [Oil, Oil, Pressure, Heat] Price: None
    --> Element: Heat, Cost: 40000, Price: 42000
    --> Element: Pressure, Cost: 40000, Price: 42000
    --> Element: Oil, Cost: 130000, Forge: [Pressure, Life, Life, Earth] Price: Some(136000)
        --> Element: Earth, Cost: 10000, Price: 10000
        --> Element: Life, Cost: 40000, Price: 42000
        --> Element: Life, Cost: 40000, Price: 42000
        --> Element: Pressure, Cost: 40000, Price: 42000
    --> Element: Oil, Cost: 130000, Forge: [Pressure, Life, Life, Earth] Price: Some(136000)
        --> Element: Earth, Cost: 10000, Price: 10000
        --> Element: Life, Cost: 40000, Price: 42000
        --> Element: Life, Cost: 40000, Price: 42000
        --> Element: Pressure, Cost: 40000, Price: 42000
--> Element: Plastic, Cost: 340000, Forge: [Oil, Oil, Pressure, Heat] Price: None
    --> Element: Heat, Cost: 40000, Price: 42000
    --> Element: Pressure, Cost: 40000, Price: 42000
    --> Element: Oil, Cost: 130000, Forge: [Pressure, Life, Life, Earth] Price: Some(136000)
        --> Element: Earth, Cost: 10000, Price: 10000
        --> Element: Life, Cost: 40000, Price: 42000
        --> Element: Life, Cost: 40000, Price: 42000
        --> Element: Pressure, Cost: 40000, Price: 42000
    --> Element: Oil, Cost: 130000, Forge: [Pressure, Life, Life, Earth] Price: Some(136000)
        --> Element: Earth, Cost: 10000, Price: 10000
        --> Element: Life, Cost: 40000, Price: 42000
        --> Element: Life, Cost: 40000, Price: 42000
        --> Element: Pressure, Cost: 40000, Price: 42000
--> Element: Rain, Cost: 40000, Price: 42000
--> Element: Metal, Cost: 130000, Forge: [Heat, Heat, Pressure, Earth] Price: Some(136000)
    --> Element: Earth, Cost: 10000, Price: 10000
    --> Element: Pressure, Cost: 40000, Price: 42000
    --> Element: Heat, Cost: 40000, Price: 42000
    --> Element: Heat, Cost: 40000, Price: 42000
```