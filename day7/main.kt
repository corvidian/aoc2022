package fi.hel.haitaton.hanke

import java.io.File
import java.util.Stack

class Dir(val parent: Dir?, val children: HashMap<String, Dir>, var size: Int)

val stack = Stack<String>()
val root = Dir(null, HashMap(), 0)

fun main(args: Array<String>) {

    File(args[0]).forEachLine { stack.push(it) }
    stack.reverse()

    var pointer = root

    while (!stack.empty()) {
        val command = stack.pop()

        if (command.startsWith("$ ls")) {
            while (!stack.empty()) {
                val content = stack.pop()
                if (content.startsWith("$")) {
                    stack.push(content)
                    break
                } else if (content.startsWith("dir")) {
                    val name = content.split(' ').last()
                    pointer.children[name] = Dir(pointer, HashMap(), 0)
                } else {
                    val size = content.split(' ').first().toInt()
                    pointer.size += size
                }
            }
        } else if (command.startsWith("$ cd ..")) {
            pointer = pointer.parent!!
        } else if (command.startsWith("$ cd")) {
            val name = command.split(' ').last()
            pointer = pointer.children[name]!!
        }
    }

    visualize(root)

    val part1 = flatten(root).filter { it < 100000 }.sum()
    println("Part 1: $part1")

    val freeSpace = 70000000 - size(root)
    println("Free space: $freeSpace")
    val smallestToDelete = flatten(root).sorted().first { freeSpace + it > 30000000 }
    println("Part 2: $smallestToDelete")

}

fun flatten(dir:Dir):List<Int> =
    dir.children.values.flatMap { flatten(it) } + size(dir)

fun size(dir: Dir): Int =
    dir.children.values.sumOf { size(it) } + dir.size

fun visualize(dir: Dir, name: String = "/", indent:Int = 0) {
    val spacer = (0..indent).map { ' ' }.joinToString("")
    println("$spacer - $name (${size(dir)})")
    dir.children.forEach{ (childName, child) -> visualize(child, childName, indent + 2) }
}
