# Bare-bones Connect Four
This is a bare-bones Connect Four project that efficiently stores the board as just two 64 bit integers. Win checking costs only 16 bit shifts and 12 bitwise `and` operations. 

This project's board state struct can be used as a library, and I will be using it for my planned upcomming Connect Four AI projects. Efficient board operations will be necessary when training Neural Networks on thousands of Connect Four games.

<img width="420" alt="Screenshot 2024-09-02 at 3 50 58 pm" src="https://github.com/user-attachments/assets/f0d9253b-90b6-4dd1-b77c-6407404fe0b9">

If you would rather play a pretty version of Connect Four over this efficient one, play https://github.com/Dot32Dev/new_connect_four 
