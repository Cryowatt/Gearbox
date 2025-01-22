# Gearbox Programmer Exam.

`Last update 1/21/2003`


## Rewrite the following function so that the following conditions are satisfied:
- the multiplication operator (`*`) is not used.
- the division operator (`/`) is not used.
- the function does not make use of loops.
- And no sneaky using assembly either!

> [Answer: `multiply_by_321()`](src/lib.rs#L8)  
> How to run:  
> ```> cargo test --doc multiply_by_321```
> 


## Write a function which takes an array of integers, and places all occurrences of the value 0 (zero) to the back of the list. Non-zero values are to appear in the same relative order.

example input:  
 `2 0 2 3 4 64 98 0 2 0`
 
example output:  
 `2 2 3 4 64 98 2 0 0 0`


### Give one solution that emphasizes clarity.

> [Answer: `simple_filter_array()`](src/lib.rs#L25)  
> How to run:  
> ```> cargo test --doc simple_filter_array```


### Give another solution that emphasizes speed.

> [Answer: `fast_filter_array()`](src/lib.rs#L48)  
> How to run:  
> ```> cargo test --doc fast_filter_array```

Solution should be in the form of C/C++ compilable source which demonstrates your solutions, using several test cases.  We expect to be able to compile this submission under msvc 6.0 and run as is.

> One could argue Rust is a "form of C/C++"? It's pretty much a descendant, and in Windows Rust compiles using the msvc toolchain. But really these requirements were written in 2003, I'm sure things have changed _slightly_ since then.


## Instruction set reference:

### `mov`

`mov mem[y], reg[x]`

performs:  
`mem[y] <- reg[x]`  

copies value in register[x] to memory location[y]

**NOTE:  can only copy from register to memory**

*example:*  
`mov mem1, reg1`


### `sub`

`sub reg[x], mem[y]`

performs:  
`reg[x] <- reg[x] - mem[y]`

subtracts the number in memory location[y] from the number in register[x] and stores the result in register[x]

*example:*  
`sub reg1, mem1`


### registers:
- reg1
- reg2


### memory locations:
- mem1
- mem2


Given the above reference, use the mov and sub instructions to move a
value from mem1 to mem2.

> [Answer: `swap_memory()`](src/lib.rs#L255)  
> How to run:  
> ```> cargo test --doc swap_memory```


## Essay:  Describe five important qualities of software.

> The answer to "what are the most important qualities of software?" is going to change depending on the job title of the person answering. Those heavily steeped in the business side of the industry will likely be laser-focused on user retention, click-through rates, conversion rates, and other KPIs. Support and ops teams, on the other hand, are loosing sleep over how many 9's they have. The PMs are in JIRA keeping an eagle-eye on deliverables and timelines, while the engineering team is trying to keep the build pipeline green with every commit. But all of these are essential the same concern: Does the software do what it's supposed to do? So obviously the functionality of software is top of the qualities list, but you need data to help prove the software is correct. This makes instrumentation the next important quality of software. Good software needs metrics and logging so the developer can know how the software is performing and potentially where it can improved. A side benefit of good instrumentation is it gives insight into the performance of software. No user wants slow and sluggish software, even if runs perfectly. The key to keeping software running well is maintainability. Good software is well structured, testable, flexible, and well documented so that it can be improved upon without collapsing like a house of cards. Lastly, I'll call the last quality "trustworthiness". Good software is one that users trust to not sell their data, leak their PII, create security vulnerabilities, or secretly install malware onto the user's system. Breaking the user's trust is the easiest way to end a business.


## Essay:  Describe what you feel is the most important aspect of the software development process.

> It's generally accepted, and I believe so as well, that the earlier phases of the process are the most important. This is usually the planning phase. The reason the earlier phases are the most important is that early changes are way cheaper than later changes. For example, changing some requirement during the testing phase would mean changing all of the tests, re-writing the code, and possibly rearchitecting the system. That can easily set a project back years. However if these issues are caught early on then the changes may only involve a whiteboad eraser.


## Is C++ slower than C?  Why or why not?

> I'd argue that they are basically the same and modern compilers are probably doing more for performance than any language differences.


## Do you have any hobby software projects?  If so, describe.

> [Tons.](https://github.com/Cryowatt?tab=repositories)
>
> My currently active projects are a C#-based game server orchestration bot for Discord and an embedded audio mixer project in Rust. Also I usually participate in [Advent Of Code](https://adventofcode.com/) every year.


## What achievement in software development are you most proud of?

> I think my proudest archievement was creating an NES emulator from scratch. Even though I didn't finish it fully, it could load mapper-0 games and I really evolved my knowlege of low-level computing concepts. It was an absolute blast and I might try a GameBoy emulator next.


## Object-oriented design
In an object oriented fashion, design the classes and hierarchy for the backpack in a game that can contain the following items: 
 - healing drink
 - combat knife
 - antidote shot
 - combat shield
 - Beretta M92 (gun)
 - Kevlar Vest

Include in the design the relationship between the classes, the hierarchy, and any member variables and methods that you think each class should contain.

> There's going to have to be a bunch of assumption I'll need to make here as the design is going to change depending on the specific game mechanics. First I'm not sure if "backpack" is a container in the game world that you can search, like how you'd find in some open world or zombie survival game. Otherwise, "backpack" could simply mean the player's inventory. Then depending on the style of game the pack might be a grid of items like in System Shock, or simply a list of items. Maybe the pack has fixed slots like in The Forest. A good middleground would be a pack that has a list of possible items, some which are stackable.
>
> ```rust
> // Using rust-ish syntax, but consider this pseudocode
>
> // Base for all game objects, would probably contain object ids from the engine
> trait GameObject {
>     entity_id: EntityId,
> }
>
> trait InventoryItem: GameObject {
>     thumbnail_asset: AssetId, // A picture of the item for the inventory UI
>     description: &str, // Short text for the inventory UI
>     max_stack_size: u16, // Max number of items of this type
> }
>
> // Items that are consumed when used
> trait Consumable {}
>
> // Items that are equipped when used
> trait Weapon {}
>
> struct Backpack {
>     // Map of items and the quanities
>     items: HashMap<EntityId, u16>
>     fn add_item()
> }
>
> struct HealingDrink: InventoryItem, Consumable {}
> struct CombatKnife: InventoryItem, Weapon {}
> struct AntidoteShot: InventoryItem, Consumable {}
> struct CombatShield: InventoryItem, Weapon {}
> struct BerettaM92: InventoryItem, Weapon {}
> struct KevlarVest: InventoryItem, Armour {}
>
> enum PackError {
>    StackFull, // Too many of this item
>    StackEmpty, // None of this item
> }
>
> struct Backpack {
>    // Adds an item to the pack, returning an error if the stack size is exceeded
>    fn add_item(item: &InventoryItem) -> Result<(), PackError>
>    // Removes a single instance of the item from the pack
>    fn remove_item(entity_id: EntityId) -> Result<InventoryItem, PackError>
>    // Possible split_stack, remove_stack functions, but depends on the design
>    // Lists the contents of the pack for the UI
>    fn list_items() -> Iter<(InventoryItem, u16)>
> }
> ```