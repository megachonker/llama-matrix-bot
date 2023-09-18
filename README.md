# Llama Matrix Bot


Welcome to the Llama Matrix Bot project, a straightforward yet powerful chat bot that enhances user interactions on the Matrix platform, utilizing the Matrix SDK. This bot uniquely leverages the LLama.cpp project directly, harnessing its capabilities to manage multiple users and workers efficiently.

At its core, the bot is designed to launch and manage multiple instances of the LLama.cpp program as "workers", facilitating simultaneous engagements and smooth operations. Despite its simple architecture, it promises a robust performance, adept at handling multiple users and orchestrating tasks across various workers seamlessly.

## Commands to use on a room
My bot for now understands and reacts to the following commands, enhancing user interaction and management:

```
!help - Displays this.
!start - Activates the bot.
!stop - Terminates the bot.
!reset - Restores the bot to its original state.
```

## Capabilities
The Llama Matrix Bot is robust and versatile, boasting the following capabilities:

- **Dynamic Room Management**: Automatically manages new rooms.
- **Multi-User Support**: Supports full parallel interactions, distributing the workload across different workers simultaneously.
- **Reset Prompt History**: Allows for the resetting of prompt history.
- **Llama Worker Scheduling**: Effectively schedules Llama workers, maintaining a consistent number of workers.
- **Multiple Worker Profiles**: Supports various worker profiles to cater to different tasks.

### Upcoming Enhancements
We are tirelessly working to upgrade the capabilities of the Llama Matrix Bot. Here are some exciting features to look forward to:

- [ ] Context Saving: Enables users to roll back and share the room state with the bot, facilitating efficient real-time scheduling by multiplexing in time.
- [ ] Workload Profile Selection: Allows users to select profiles based on workload necessities.
- [ ] GPU Support: Aiming to introduce GPU support to boost computational speed and performance.
- [ ] Horizontal Scaling: Designed to run on multiple hardware devices.
- [ ] User-Centric Features: We are planning to unveil numerous user-friendly features to further enhance the user experience.

## Specifications
The Llama Matrix Bot is segmented into several components, each spearheading a specific functionality, ensuring a smooth operational flow:

- **Configuration Parser**: A module devoted to parsing YAML files containing login details and settings.
- **Worker Lifecycle Management**: Handles the spawning and management of worker lifecycles, guaranteeing seamless functionality.
- **Bot Interaction and Scheduling**: Manages the bot's interaction with the Matrix platform and efficiently orchestrates worker scheduling.

This bot leverages the extensive capabilities of the Tokio framework, utilizing asynchronous functionalities, Automatic Reference Counting (ARC), and mutex locks, combined with the prowess of MSCP (Message Passing Control Protocol), to offer a stable and efficient solution.
