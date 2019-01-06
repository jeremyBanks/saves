const Discord = require('discord.js');

const { DISCORD_TOKEN } = process.env;
if (!DISCORD_TOKEN) {
    throw new Error("DISCORD_TOKEN not set");
}

const client = new Discord.Client();
client.login(DISCORD_TOKEN);

const message = process.argv.slice(2).join(' ');
if (!message.trim()) {
    throw new Error("message not specified");
}

client.on('ready', () => {
    client.channels.get(
        '303217117589667840' || '514230161063804928'
    ).send(message).then(() => process.exit(0));
});
