---
title: Portfolio
category: portfolio
tags:
- Programming
---

# Larger applications

## MilestoneMap

![Milestone Map screenshot]({{ '/img/milestoneMapScreen.png' | relative_url}})

An application used to track project and programme delivery, and display this information in an easy to read and interpret format. MilestoneMap allows a project manager to communicate and compare otcomes to stakeholders.

MilestoneMap can be used as a single page web app on GitHub [here](https://andymac-2.github.io/milestoneMap/)

`JavaScript`, `HTML`, `SVG`, `CSS`.

## TrakMap

A sister application to Milestone Map. TrakMap allows a project manager to communicate dependencies, critical products, and scheduling to stakeholders. The main aim of TrakMap is to produce easy to read and interpret diagrams.

TrakMap can be used as a single page web app on GitHub [here](https://andymac-2.github.io/trakMap/)

`JavaScript`, `HTML`, `SVG`, `CSS`.

## Gloop

![Gloop cow]({{ '/img/gloopScreen.png' | relative_url}})

Gloop is a platformer game made for the November 2018 Github game jam. The game was created in a single month using Godot and GDScript. This game features original artwork with colourful landscapes, puzzles, multiple power-ups, and natural physics.

Gloop placed 13th overall out of approximately 330 submissions.

Gloop can be played directly in the browser, or downloaded at itch.io [here](https://pilotinpyjamas.itch.io/gloop)

`Godot`, `GDScript`, `GIMP`.

## Raytrace

![Raytrace Screen]({{ '/img/raytraceScreen.png' | relative_url}})

Raytrace is a simple path tracing program written in Rust using a physical light model. Rendering uses fresnel equations to accurately simulate the natural path and attenuation of light as is passes through and reflects off objects. Caustics and global illumination are emergent properties of the physical equations used to describe the lighting model.

`Rust`, `PPM`.

# Toys, proof of concept, and smaller applications

## Asteroids

![Asterids Clone]({{ '/img/asteroidsScreen.png' | relative_url}})

Asteroids is a clone of the original Asteroids arcade cabinet game written in Haskell using SDL for IO. Functional programming makes it easy to create well structured applications, leading to a clean and strictly hierarchical design. This is an example of these design patterns.

Asteroids can be seen on GitHub [here](https://github.com/andymac-2/asteroids)

`Haskell`, `SDL`.

## Fourier Polygon

![Fourier transform]({{ '/img/fourierPolygonScreen.svg' | relative_url}})

Joseph Fourier was a french mathematician that proved that any complex, repeating signal could be decomposed into complex exponentials. We use this property to show that we can draw arbitrary shapes using series of circles rotating at a constant angular velocity. We can think of each circle as a cog in a complex machine, or as a series of clock hands spinning at different rates. This program also demonstrates that in certain degenerate cases, interpolation between individual points is often not ideal.

The user can provide a series of points to draw any shape they like, and this library exposes a simple API to embed a canvas in any webpage.

Fourier Polygon can be demo'd [here](https://andymac-2.github.io/fourier-polygon/). A more detailed writeup can be found [here](https://github.com/andymac-2/fourier-polygon)

`JavaScript`, `SVG`, `CSS`, `Fourier Transform`.

## Vectors in C

C lacks many features of modern languages, including generic, typesafe data structures. This library is a demonstration of how to monomorphise generic data structures by including C files multiple times. Some macros are required to concatenate function names together. Vectors created using this library are typesafe, and specialised for each declared type.

The C Vector library can be seen on GitHub [here](https://github.com/andymac-2/c_vector)

`C`.

## SubWorld

```
start:
    # putchar until null terminator
    helloString -1 -1;

    # increment text pointer
    data.1 start;

    # goto start
    data.0 data.0 start;

helloString:
    "Hello World!\n\0";

data:
    [0x00, -1];
```

An esoteric programming language consisting of only two symbols: `hello` and `world`. We provide a `C` runtime. Because programs in this language are so notoriously difficult to write, A compiler from a more sane language written in Haskell is provided. The input for this compiler is shown above.

`Haskell`, `parsing`.

