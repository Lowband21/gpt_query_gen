<script>
    import { onMount } from 'svelte';
    let messages = [];
    let input = "";
    let error = "";

    async function getGptResponse(chatData) {
        try {
            let response = await fetch("/api/chat", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(chatData)
            });

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }

            let data = await response.json();
            console.log(data);
            return data;
        } catch (err) {
            error = err.message;
            console.error("Error occurred during fetch:", err);
        }
    }

    async function addMessage() {
        if (!input) return;
        messages = [...messages, { text: input, user: true }];
        let response = await getGptResponse({ messages });
        input = "";
        if (response) {
            messages = [...messages, { text: response, user: false }];
        } else {
            console.error("Error occurred while getting GPT-3 response. Error: ", error);
        }
    }
</script>

<div id="chat-window">
    {#each messages as message (message)}
        <div class={message.user ? "user-message" : "gpt-message"}>
            {message.text}
        </div>
    {/each}
</div>
<form on:submit|preventDefault={addMessage} class=prompt>
    <input bind:value={input} placeholder="Type a message...">
    <button type="submit">Send</button>
</form>

<style>
    #chat-window {
        max-width: 90%;
        height: 80%;
        overflow: auto;
        border: 3px solid #000000;
        padding: 10px;
        margin-top: 10px;
        margin-bottom: 10px;
        margin-right: auto;
        margin-left: auto;
        background-color: #333333;
        border-radius: 5px;
        justify-content: center;
        justify-self: center;
    }
    .prompt {
        display: flex;
        align-items: center;
        justify-content: center;
    }
    
    .user-message, .gpt-message {
        margin: 5px;
        padding: 10px;
        border-radius: 5px;
    }
    
    .user-message {
        background-color: #a8dadc;
        align-self: flex-end;
    }
    
    .gpt-message {
        background-color: #f1faee;
    }
</style>