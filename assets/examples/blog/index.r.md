Credence: Blog Example
======================

Welcome to my amazing blog!

Here I'll be sharing my deep thoughts with the world.

And maybe even with visitors from other worlds. LOL.

* [Opinions](opinions)
* [Food](food)

Tables Are Cool
---------------

| Tables   |      Are      |  Cool |
|----------|:-------------:|------:|
| col 1 is |  left-aligned | $1600 |
| col 2 is |    centered   |   $12 |
| col 3 is | right-aligned |    $1 |

[Mermaid](https://mermaid.js.org/) Is a Zillion Times Cooler!!!!
----------------------------------------------------------------

<pre class="mermaid">
    ---
    title: Animal example
    ---
    classDiagram
        note "From Duck till Zebra"
        Animal <|-- Duck
        note for Duck "can fly<br>can swim<br>can dive<br>can help in debugging"
        Animal <|-- Fish
        Animal <|-- Zebra
        Animal : +int age
        Animal : +String gender
        Animal: +isMammal()
        Animal: +mate()
        class Duck{
            +String beakColor
            +swim()
            +quack()
        }
        class Fish{
            -int sizeInFeet
            -canEat()
        }
        class Zebra{
            +bool is_wild
            +run()
        }
</pre>
