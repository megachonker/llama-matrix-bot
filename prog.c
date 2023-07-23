#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>

int main()
{
    while (1)
    {
        /* code */
        char uwu[] = "UWU\n";
        char buffer[3+sizeof(uwu)];
        memset(buffer,'\0',sizeof(buffer));
        ssize_t bytesRead=0;

        // Read from standard input
        bytesRead = read(STDIN_FILENO, buffer, sizeof(buffer)-sizeof(uwu));

        if (bytesRead < 0)
        {
            perror("Error reading from stdin");
            return 1;
        }

        // Wait for 1 second
        sleep(1);

        // Concatenate "UWU" to the input
        strcat(buffer, uwu);

        // Write to standard output
        ssize_t bytesWritten = write(STDOUT_FILENO, buffer, sizeof(buffer));

        if (bytesWritten < 0)
        {
            perror("Error writing to stdout");
            return 1;
        }
    }
    return 0;
}
