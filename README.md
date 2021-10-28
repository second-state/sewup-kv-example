# sewup-kv-example
A key value feature example for sewup hackthon

## Hangman Game
Hangman is a classic game for learning English words.

If you are not familiar with this game, you can play this [web game](https://www.gamestolearnenglish.com/hangman/).

In this example, we design a game including the basic idea of Hangman and use sewup kv feature as backend.

Here is the rule, and we will implement this latter.
1. Every account can set only a word as the puzzle, a sentence as the reward for people solve the puzzle, and a hint for people
2. Everyone can get the information of the puzzle (the length of words and the hint).
3. Everyone guess one char, for example `p`, and he will get a partial string like `-pp--`.
4. Everyone can guess the word, if correct, he will see the sentence.
