# cs-3310-kosaraju

Implementation of Kosaraju's algorithm for finding strongly connected components for UVU CS 3310 Analysis of Algorithms. Designed as a solution to Tim Roughgarden's Algorithms Illuminated problem 8.10.

Using the existing implementation of graphs in Rust provided by petgraph, but redoing the algorithm part (including DFS).

## Usage

Run without arguments for sanity checks. Pass a single filename to read in that file as a graph. See test data in [data](data/) for examples of the format.

## Lessons Learned

This was a pretty frustrating assignment. It seems Mr. Roughgarden has set us up for failure here. The textbook only goes over the recursive algorithms for Kosaraju, not the iterative ones. And then the challenge data set has 5 million edges and 800k verticies, which will obviously overwhelm any reasonable recursion depth. So you need to come up with the iterative algorithms or find them described elsewhere.

That by itself wouldn't be so bad, and perhaps I would have been more receptive to this untelegraphed point of friction had I been using a language and data structures I was familiar with. But what actually happened was much different: I saw that all of the component algorithms had their psuedocode described in good detail in the textbook and figured that it might be a bit of a boring one if I went with things I was familiar with. So I decided to use Rust and `petgraph` to build skills in some ancilliary areas of interest. I also recognized up front that recursion wouldn't work great and so went into this intending to just convert the recursive DFS into iterative versions.

I figured I could skip the psuedocode step because the book gave me the psuedocode--somehow skipping over the fact that I was adjusting the algorithms. Lesson one: Don't skip writing the psuedocode. So I implemented iterative DFS before realizing there's no logically equivalent place to put the order assignment in the iterative version. Lesson two: Iterative and recursive algorithms can't simply be substituted for each other in all cases. Now, yes, you can make an iterative algorithm out of a recursive one, but as this wasn't an example of tail-call recursion, it is nontrivial to do so.

By this point I had spent a lot of brainpower on this assignment just in working with Rust and `petgraph` and learning the APIs and such. So I didn't want to think about efficiently unrolling the recursive algorithm into one making use of an explicit stack. So I thought, why reinvent the wheel? Surely iterative topological sorts (the first use of DFS in this algorithm) have been done before. I found some great psuedocode for Kahn's algorithm, which is a very intuitive algorithm for topological ordering. Only after implementing it did I realize that, when applied to cyclic graphs, it completely fails (whereas DFS produces a psuedo-topological sort). Lesson 3: Double check the parameters of your algorithms before you start trying to implement them.

I spent a moment trying to understand closures as I was afraid that recursive functions with mutable references would cause a borrow checker issue. It doesn't. In my defense, my brain was already a little fried from all the failed attempts thus far, and also Rust doesn't have the convenient interactive sessions I'm used to with Python. Also the book used globals, which I was smartly avoiding by passing things through.

Anyway, I got the thing working with the recursive version that the book has, but it is super slow and invariably runs into the recursion limit. I tried changing it but I couldn't find any effect when testing on a minimal example of recursion so idk what's up with that. 

Defeated at this point, I turned to my groupmates for help. One of them had done the work to unroll the recursive algorithm into an explicit stack. So I translated their code into Rust and also to work with the data structures I was already using. And it worked--thank goodness. But it also takes 40 minutes to run. Which my friends' in Python did not. Lesson four: Using a "faster" language doesn't mean crap if you use the wrong data structures for the job.

I got my grade on the assignment but I kinda want to go back and use better-fitting data structures to speed it up. I'm confident I can get on the level of my groupmates, if not faster, but. I already know premature optimization is the root of all evil. And while I have my suspicions about what's so slow (using `.iter().position()` to sync indicies every time, therefore creating an 800k items iterator every time, rather than using a hashmap or similar), I don't know for sure. And my only test cases are on a dozen verticies, which might be nonrepresentative of larger datasets, and 800k verticies, which takes, again, 40 minutes to process and presumably longer to benchmark. Creating ideal test data for this is nontrivial as well, given that they need to have a decent number of strongly connected components without just becoming one strongly connected component.

So, we'll see, I guess, if I return to this to properly optimize it.
