# Fluffy Tribble #

This is what happens when I get so confused by nPhysics that I decide to try and roll my own.  This will probably be a disaster. :)

I will try and keep it simple.

Also, the only reason why I'm creating this is so it can be integrated into a Web Assembly project.  So that's the main target.

## The State of Things ##

* The physics system is fully impulse-based.
* It can prevent objects from penetrating.
	* This implements restitution coefficients so things can have different amounts of riccochet.
* It can handle linear and angular displacement/velocity/etc.
* Basic friction was just implemented, so now things can stick in place and sliding will cause rolling.
	* The friction is separated into static and dynamic (using basically the "Coulomb friction-cone model").  This is mostly a cheat to handle how impulse-based physics is not great at tracking how long things are "in contact": if there's a lot of relative motion that the friction would oppose, the system uses the lower dynamic friction coefficient to simulate the objects only being in contact briefly.  It will switch to the static friction coefficient in the opposite case to simulate skidding across a surface.
* Entities will be put "to sleep" if their total energy falls below a given threshold.  In this state, they ignore collisions with thing they're resting against.  This is mainly so that they can come fully to rest, and won't slowly sink into surfaces.

The colliders may not be 100% setup yet.  I'm trying to avoid collision detection that is iterative (i.e. like GJK + binary search).  Instead I often rely on linear approximations to convert the problems into ones with closed-form solutions.  This means that **large time-steps could lead to rotation-based collisions being lost!**  On the plus side, this also means that all collision handling is implicitly continuous.

Here are the collider combinations that should work so far:

|                   | Sphere | Plane<sup>1</sup> |           Mesh          |
|-------------------|--------|-------------------|-------------------------|
| Sphere            |  DONE  |        DONE       |           DONE          |
| Plane<sup>1</sup> |  DONE  |  N/A<sup>2</sup>  |           DONE          |
| Mesh              |  DONE  |        DONE       | IN PROGRESS<sup>3</sup> |

Notes:

1. There are a few things worth noting about plane colliders:
	* They are _infinite_ planes.  These bisect space and only allow things to move freely on one side of the plane.  These are intended to be used as "world boundaries" to keep things within some convex area.
	* They aren't intended to handle rotations.
2. Since planes are infinite, it doesn't really make sense to try and collide them.  They're either parallel, or they collide somewhere.
3. Mesh-mesh collision is basically two types of check: (1) collide the verticies of one mesh against the surfaces of the other, and (2) collide the edges agasint eachother.  The first is functioning.  The second is a TODO item.  In this state, collision _mostly_ works, but things can definitely still glitch into eachother.

**Warning:** Broad-phase collision handling hasn't been setup yet.  So everything is basically `O(N^2)` right now.

## Why the Name "Fluffy-Tribble"? ##

GitHub recommended it. I figured it was silly, much like this project, so why not?

Didn't realize it was an actual thing from _Star Trek_ until after the first commit. Probably would've made the name even more nonsense if I had known that.
