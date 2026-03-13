<script lang="ts">
  import { messages, formatTime, formatFullDate, type Message } from '$lib/stores/messages';
  import { selectedConversation } from '$lib/stores/conversations';
  import { onMount, tick } from 'svelte';

  let messageContainer: HTMLDivElement;
  let isLoading = $state(false);

  // Load messages when conversation changes
  $effect(() => {
    const conv = $selectedConversation;
    if (conv) {
      messages.load(conv.id);
    } else {
      messages.clear();
    }
  });

  // Scroll to bottom when messages change
  $effect(() => {
    const _ = $messages; // Subscribe to messages
    tick().then(() => {
      scrollToBottom();
    });
  });

  function scrollToBottom() {
    if (messageContainer) {
      messageContainer.scrollTop = messageContainer.scrollHeight;
    }
  }

  async function loadMoreMessages() {
    if (!$selectedConversation || $messages.length === 0 || isLoading) return;
    
    isLoading = true;
    const oldestMessage = $messages[0];
    try {
      await messages.loadMore($selectedConversation.id, oldestMessage.sent_at);
    } finally {
      isLoading = false;
    }
  }

  function isSameDay(msg1: Message, msg2: Message): boolean {
    const d1 = new Date(msg1.sent_at);
    const d2 = new Date(msg2.sent_at);
    return d1.toDateString() === d2.toDateString();
  }

  function shouldShowDate(msg: Message, index: number): boolean {
    if (index === 0) return true;
    return !isSameDay(msg, $messages[index - 1]);
  }
</script>

<div class="conversation-view">
  {#if $selectedConversation}
    <div class="conversation-header">
      <h3>{$selectedConversation.title || 'Unnamed Conversation'}</h3>
      <div class="header-actions">
        <button 
          class="action-btn"
          onclick={() => messages.sync($selectedConversation.id)}
          disabled={isLoading}
        >
          {isLoading ? 'Syncing...' : 'Sync'}
        </button>
      </div>
    </div>

    <div class="messages-container" bind:this={messageContainer}>
      {#if $messages.length > 10}
        <button class="load-more" onclick={loadMoreMessages} disabled={isLoading}>
          {isLoading ? 'Loading...' : 'Load older messages'}
        </button>
      {/if}

      {#each $messages as message, index (message.id)}
        {#if shouldShowDate(message, index)}
          <div class="date-divider">
            <span>{formatFullDate(message.sent_at)}</span>
          </div>
        {/if}

        <div 
          class="message"
          class:from-me={message.is_from_me}
          class:reply={message.reply_to_message_id}
        >
          <div class="message-content">
            <div class="message-header">
              {#if !message.is_from_me}
                <span class="sender-name">{message.sender_name || 'Unknown'}</span>
              {/if}
              <span class="message-time" title={formatFullDate(message.sent_at)}>
                {formatTime(message.sent_at)}
              </span>
              
              {#if message.edited_at}
                <span class="edited-mark">(edited)</span>
              {/if}
            </div>
            
            <div class="message-body">{message.content}</div>
          </div>
        </div>
      {:else}
        <div class="empty-state">
          <p>No messages yet</p>
          <p class="hint">Send a message to start the conversation</p>
        </div>
      {/each}
    </div>
  {:else}
    <div class="no-selection">
      <div class="no-selection-content">
        <div class="icon">💬</div>
        <h3>Select a conversation</h3>
        <p>Choose a conversation from the sidebar to start messaging</p>
      </div>
    </div>
  {/if}
</div>

<style>
  .conversation-view {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: #f9fafb;
  }

  .conversation-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.25rem;
    background: white;
    border-bottom: 1px solid #e5e7eb;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
  }

  .conversation-header h3 {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
    color: #111827;
  }

  .header-actions {
    display: flex;
    gap: 0.5rem;
  }

  .action-btn {
    padding: 0.375rem 0.75rem;
    font-size: 0.8125rem;
    color: #6b7280;
    background: white;
    border: 1px solid #d1d5db;
    border-radius: 0.375rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .action-btn:hover:not(:disabled) {
    background: #f3f4f6;
    color: #374151;
  }

  .action-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .messages-container {
    flex: 1;
    overflow-y: auto;
    padding: 1rem 1.25rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .load-more {
    align-self: center;
    padding: 0.5rem 1rem;
    font-size: 0.8125rem;
    color: #6b7280;
    background: white;
    border: 1px solid #d1d5db;
    border-radius: 1rem;
    cursor: pointer;
    transition: all 0.2s;
    margin-bottom: 0.5rem;
  }

  .load-more:hover:not(:disabled) {
    background: #f3f4f6;
    color: #374151;
  }

  .load-more:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .date-divider {
    display: flex;
    align-items: center;
    justify-content: center;
    margin: 1rem 0;
  }

  .date-divider span {
    font-size: 0.75rem;
    font-weight: 500;
    color: #6b7280;
    background: #e5e7eb;
    padding: 0.25rem 0.75rem;
    border-radius: 1rem;
  }

  .message {
    display: flex;
    max-width: 70%;
    align-self: flex-start;
  }

  .message.from-me {
    align-self: flex-end;
  }

  .message-content {
    background: white;
    padding: 0.625rem 0.875rem;
    border-radius: 0.75rem;
    border-bottom-left-radius: 0.25rem;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
    max-width: 100%;
  }

  .message.from-me .message-content {
    background: #0078d4;
    color: white;
    border-bottom-left-radius: 0.75rem;
    border-bottom-right-radius: 0.25rem;
  }

  .message-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.25rem;
  }

  .sender-name {
    font-size: 0.8125rem;
    font-weight: 600;
    color: #0078d4;
  }

  .message.from-me .sender-name {
    display: none;
  }

  .message-time {
    font-size: 0.6875rem;
    color: #9ca3af;
  }

  .message.from-me .message-time {
    color: rgba(255, 255, 255, 0.8);
  }

  .edited-mark {
    font-size: 0.6875rem;
    color: #9ca3af;
    font-style: italic;
  }

  .message.from-me .edited-mark {
    color: rgba(255, 255, 255, 0.7);
  }

  .message-body {
    font-size: 0.9375rem;
    line-height: 1.4;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    flex: 1;
    color: #6b7280;
    text-align: center;
  }

  .empty-state p {
    margin: 0;
  }

  .empty-state .hint {
    font-size: 0.875rem;
    margin-top: 0.5rem;
    color: #9ca3af;
  }

  .no-selection {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    background: white;
  }

  .no-selection-content {
    text-align: center;
    color: #6b7280;
    padding: 2rem;
  }

  .no-selection-content .icon {
    font-size: 3rem;
    margin-bottom: 1rem;
    opacity: 0.5;
  }

  .no-selection-content h3 {
    margin: 0 0 0.5rem 0;
    color: #374151;
    font-size: 1.125rem;
  }

  .no-selection-content p {
    margin: 0;
    font-size: 0.9375rem;
  }
</style>
