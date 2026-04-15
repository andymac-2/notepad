# TODO

- Eve SSTO
- Reusable boosters
- gravity boosts

# Highest and lowest altitudes

https://forum.kerbalspaceprogram.com/topic/173446-lowest-highest-points-of-celestial-bodies/

# General

- Low Kerbin orbit is about 2200 m/
- Press O on the keyboard to put on your helmet. Kerbals inside a firing will die without it.

# Delta v guide corrections

- Ike has a safe minimum altitude of 29km not 10km
- Moho has a safe minimum altitude of 16km, not 50km
- Minmus plane change is ~40m/s not 340m/s

Moho -> kerbin
- Start with 3393
- 141m/s to get to SOI edge
- 2733 eve encounter 660m/s from moho down from 2400
- 2600 to kerbin encounter


# Delta V cheatsheet

- low Laythe orbit -> Laythe SOI edge: 600m/s
- Laythe SOI edge -> Tylo: 60m/s
- Low Moho orbit -> Moho SOI edge: 310m/s


Technique 1:

- Tylo capture, set Pe with som margin above Jool
- Laythe assist to get a smaller orbit and another tylo assist
- Use the next Tylo assist to set the Pe/Ap and inclination at the level of your planet
- Capture burn at your moon
- Budget 200ms of delta v??? + capture burn

# Science

- There are 5 altitudes for science: Space high, Space low, Flying high, Flying low, and Surface.
    - Flying is only available if there is an atmosphere
- Every science experiment can only be performed once in space high, except for the gravioli detector, which is per biome.
- Every science experiment can only be performed once in space low except for the gravioli detector and EVA report which is per biome.
- Every science experiment can only be performed once flying high except for the atmosphere analysis which is per biome.
- Every sciecne expermient can be performed once per biome flying low except for the mystery goo and barometer.
- Every sciecne expermient can be performed once per biome on the surface except for the eva experiment kit.

# IVA info

- Use pahse angle to get an intercept with a planet
- An inclination of 0 degrees is east
- Mun -> Kerbin ejection angle 32 degrees (target at 32 degrees pitch on navball), with an extra 50m/s of dV.
- Minmus -> Kerbin ejection angle is about 90 degrees. Just burn at lowest velocity.

# Spaceplane design

- Landing gear in tripod formation: 1 at front, 2 in the rear
    - Rear landing gear behind COM
        - Too far forwards and you tail strike or fall backwards
        - Too far back and you bounce when you land
    - Forward landing gear should be higher than rear
- Centre of lift should be behind COM
    - It needs to be about a balls width behind
    - Too low and the craft will become unstable
- Lift surfaces should have AoA built in
- Elevators should be as far rear as possible
- Ailerons should be as wide as possible
- Brakes set to max braking
- Max 4 rapiers per shock cone intake
- SAS OFF when flying air breathing, use option + WASD for trim instead
- Roll instability can be caused by too much aileron authority
- Roll instability can be casued by COG too high.
    - This also happens when you pitch up

# Standard actions

- Use F6 and F7 to switch between action sets

|  | Default Action Set   | Action set 1         | Action set 2         | Action set 3         | Action set 4         |
|--|----------------------|----------------------|----------------------|----------------------|----------------------|
| 1| Perform Science      |                      | Engine Mode 1        | Toggle Drill         |                      |
| 2| Collect Science      |                      | Engine Mode 2        | Surface Harvester    |                      |
| 3| Dispose Science      |                      | Engine Mode 3        | Toggle LF+Ox         |                      |
| 4| Toggle Panels        |                      |                      | Toggle Panels        |                      |
| 5| Tgl. Pnls. Subcraft  |                      |                      | Toggle Fuel Cell     |                      |
| 6| Toggle Antenna       |                      |                      |                      |                      |
| 7| Engine Mode 1        |                      | Translate Left       |                      |                      |
| 8| Engine Mode 2        |                      | Translate Down       |                      |                      |
| 9| Disposable Science 1 |                      | Translate Up         |                      |                      |
|10| Disposable Science 2 |                      | Translate Right      |                      |                      |


# Eve ascender

## Flying

- SAS prograde
- Never let prograde marker drop below horizon unless descending.

## Landing procedure

- Stage off any extra weight
- SAS radial out
- Ensure speed < 30m/s to ensure panels do not break off
- Extend panels. **Do not quicksave until landing unless you want the panels to break off!**
- Autorotate until about 800m
- Full throttle until speed has dropped below 10m/s
- Throttle down to maintain 10m/s
- When 25m AGL, full throttle
- Cut throttle when close to ground
- Go around by maintaining throttle

When slowing down, use full throttle and then throttle back, rather than slowly throttling up from zero.

## Ascending

## Propellor stage

Ensure you stay below the following speeds to prevent solar panels from breaking:

| Altitude | Speed | Speed (Kerbin) |
|----------|-------|----------------|
| 0km      | 30m/s | 70m/s          |
| 5km      | 35m/s | 100m/s         |
| 10km     | 45m/s |                |
| 15km     | 60m/s |                |
| 20km     | ???   |                |

- Radial out, full throttle
- Stage extra weight
- Maintain 440RPM max
- At 42km, change to stability from radial out to avoid surface to orbit transition

## Rocket stage

- Open fuel tank dialogs
- SAS to stability
- Speed mode to Orbit
- Full throttle
- Stage, and immediately SAS to stability again
- Pitch to 75 degrees
- Maintain 45s to apoapsis with pitch

Launch profile once at max prop altitude:


| Pitch (degrees) | Speed (m/s) |
|-----------------|-------------|
| 60              | 500         |
| 50              | 230         |
| 45              | 280         |
| 40              | 330         |


# Safe aerobraking altitudes

High bound means we can try 1-2km less next time. The lower bound is when the craft explodes or we feel uncomfortable going any lower
 
| Body      | Apoapsis    | Pe Hi/Lo bound |
|-----------|-------------|----------------|
| Kerbin    | 550k        | 33k/??k        |
| Kerbin    | 850k        | 37k/??k        |
| Kerbin    | 2M          | 38k            |
| Kerbin    | 5M          | 41k            |
| Kerbin    | 14M (Mun)   | 44k            |
| Kerbin    | 46M (Minmus)| ??k/??k        |
| Kerbin    | Eve         | 48k/46k        |
| Duna      | Kerbin      | 28k/??k        | Below 26k, you will land
| Duna      | 24M (SOI)   | 30k/??k        | Below 30k, you will probably land
| Duna      | 15M         | 30k/??k        |

# Minmus departures

| Destination    | Angle CCW from ejection |
|----------------|-------------------------|
| Jool (1000m/s) | 63                      |

## Laythe

- SOI
    - 40km too low, shock cones almost overheated, but got into low Laythe orbit anything below this and you will land

## Kerbin

- From Jool
    - 7km/s re-rentry: 38km altitude is too high. The craft skipped right over.

# MISTAKE HALL OF FAME

- Saved four kerbals in a hitchiker, not enough parachutes, 8m/s insterad of 6m.s at landing, everyone died. Rescue mission took 2 hours.
- Sent probe to Eve, forgot to extend solar panels, in correct orbit, dead.
- The shutdown temperature for the convert-o-tron 125 is 1500K, but a science Jr. will explode at 1200K. Don't connect the reactor directly to a science Jr.
- Engineer level 0 cannot repack parachutes. Make sure if you hire a new astronaut, they orbit kerbin first to get a star.
- The light scanner arm only transmits 33% science, even if you find another of the same formation.
- You have to enable the sentinel scanner to make it scan.
- A pod without a heat shield is aerodynamically unstable, and will tend to point nose down
- Use drogue chutes. They can be partially deployed at high velocity, and can help upright a nose down ship. Partially deployed, they have more drag than a main chute.
- Don't skip the checklists
- If you start a rescue mission and a "dock two vessels in orbit" mission, and send an egineer to attach a docking port to the wreckage and dock with it, it does not count as two different vessels.
- Aseroids have shade, make sure you're on the sun side of the asteroid or have a NUK

# Advice

- When creating a manual alarm, it only allows you to input a relative time. If you want to input an absolute time, you can save the alarm. When you edit the alarm, the time input will change to absolute.
- Autostrut is only available after you unlock struts
- Crossfeed and fuel flow priority is only available if you unlock fuel systems in the tech tree
- Kerbals can only deploy their personal chutes if they have at least one star
    -  Rescued kerbals have one start by default if you EVA them. If you use the claw on their craft and never directly control them, they end up with 0 stars.
- Align maneuvers before you time warp.
    - If you have a slow turning vessel, you won't miss your mark this way.
    - If your unmanned probe loses power, you will still be able to execute your maneuver.
- Extend your solar panels before you time warp
- Avoid sending a manned craft for a rescue mission whenever possible: you may need to rescue two kerbals that way.
- For lifting: Kodiaks are the cheapest liquid fuel engine per unit thrust, Bobcats are the cheapest if you want a gimbal
    - Two Bobcats equals a Skipper, three equals a Mainsail.
- For transfer NERV is the clear winner overall
- For transfer with checmical engines: the Ant, Spark, Terrier, Cheetah, and Wolfhound have the highest ISP/weight in a vacuum. Which one you choose depends on the weight of your craft.
    - The chetah beats the terrier at around 28t
- If you make a mistake designing your craft/have the wrong crew, you can recover your craft on the launchpad without launching it for a full refund.
- If you have a problem with a mission, walk away and do another one. You make more mistakes when you're tilted. Come back to that mission later fresh.
- It's OK to fail a contract sometimes.
- Engineers are the most useful kerbal, don't use them unnecessarily.
- Practice aborts ahead of time.
- You need two fuel lines, one going each way to transfer fuel accross a heat shield/claw
- Enter the atmosphere contracts can be done with debris. e.g. eject an empty fuel tank and be in control of it and it will pass the contract.

# Principia

- Timewarp always succeeds to exactly one minute.

- Jank:
    - I normally use the maneuver editor in the bottom left corner, which is full of jank. In stock KSP, sometimes the maneuver disappears, you can't see the total delta V for any maneuver except maneuver 1, it stays grey for a while and then opens. Principia has none of these issues.
    - The rebase functionality is amazing in principia, and allows you to perform cowboy flying without maneuver nodes seamelessly with maneuvers.
    - The Map screen in KSP is complete jank when intercepting multiple orbits in advance, sometimes the conics flicker
