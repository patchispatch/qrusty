# Qrusty

![](pics/krusty.jpg)

An attemp on re-implementing [fdavidcl's q](https://github.com/fdavidcl/q) in Rust for learning purposes, with added functionalities.

## What's this?

This is a simple CLI program to save plain text in a queue. You can use it as a watch list, a to-do list... Whatever you want.

## How it works

Currently this is the most simple thing in the universe. You call ```qrusty``` with an argument, the program saves it in the queue.
You call it without arguments, it shows you the oldest element you saved and deletes it. 

The queue is saved on a txt file, by default at "./q.txt". 

## The plan

The plan is to make things a little more complex, including:
- Using a NoSQL database to manage the list
- Possibility to add tags, notes, etc.
- Showing a sneak peek of the list
- Filtering by name and/or tags
- Organizing the code with several types and traits
- Whatever I may add in the future to feel better with myself

