# Ormen Långe
Place one card from the deck at the rightmost position on the board. 
If there is a card immediately to the left of the placed card which share a suite or value 
with the placed card, they can be merged. Alternatively, if there is a card of the same suite or 
value 3 steps to the left of the placed card (if there are two cards between them), those can be merged as well. 

Whenever possible, the player has to merge cards. The merging goes from right to left, meaning the rightmost card of any merge 
will be at the top of the pile. from then on, the pile is only representd by its top card and moves as one pile whenever merged with anohter.

The game is won if only one pile remains when the entire deck has been played.

# The Problem
What is the best possible way to play Ormen Långe, and what percentage of games can we expect to solve?

# The Project
This project is 100% Rust based. There is an interface to play the game.
The big problem is making an exceptional AI.
