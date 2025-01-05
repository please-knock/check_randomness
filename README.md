\nThis is a simple code for checking randomness. \n\n[Option 1] The random counter uses the randomness() function to first generate a random number, and then generates a second random number which is inside a loop with a counter. Endlessly comparing two random numbers until they match each other will give a third random number. Then the random_counter() function will loop randomness() a million times, and then add those all, and then divide it by a million to get the average. The average will always be close to 100 because that is just the way how it works. I haven't yet came up with a clear understanding of this code too. However you can somewhat prove it by executing the second function. \n\n[Option 2] Function highest_counter() will execute randomness() inside a loop and only print the results (which is the number of endless tries until the first generated random number matches the second generated random number) that are higher than the previous. You will also see how much time has passed since the execution. It takes less than one second to reach 1000, while from 1000 it takes significant amount of time. Theoretically, there is no end to this calculation. In other words, there is a possibility that first generated random number and second generated random number never becomes identical. \n\n[Option 3] This will loop randomness() and print the outputs without doing any other operations with it. You will see that displayed numbers are usually around 1 to 100. Numbers that are higher than 1000 will be colored in yellow.\n
