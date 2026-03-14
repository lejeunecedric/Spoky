<script lang="ts">
  import { messages } from '$lib/stores/messages';
  import { selectedConversation } from '$lib/stores/conversations';
  import type { Message } from '$lib/stores/messages';

  interface Props {
    replyingTo?: Message | null;
    onCancelReply?: () => void;
  }

  let { replyingTo = null, onCancelReply }: Props = $props();

  let messageText = $state('');
  let isSending = $state(false);
  let inputRef: HTMLTextAreaElement;

  async function handleSend() {
    const content = messageText.trim();
    if (!content || !$selectedConversation || isSending) return;

    isSending = true;
    try {
      await messages.send($selectedConversation.id, content, replyingTo?.id);
      messageText = '';
      
      // Clear reply state after sending
      if (replyingTo) {
        onCancelReply?.();
      }
      
      // Reset textarea height
      if (inputRef) {
        inputRef.style.height = 'auto';
      }
    } catch (e) {
      console.error('Failed to send message:', e);
    } finally {
      isSending = false;
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      handleSend();
    }
    if (event.key === 'Escape' && replyingTo) {
      event.preventDefault();
      onCancelReply?.();
    }
  }

  function autoResize(event: Event) {
    const target = event.target as HTMLTextAreaElement;
    target.style.height = 'auto';
    target.style.height = Math.min(target.scrollHeight, 120) + 'px';
  }
</script>

<div class="message-input">
  {#if $selectedConversation}
    {#if replyingTo}
      <div class="reply-preview">
        <div class="reply-info">
          <span class="reply-label">↩ Replying to {replyingTo.sender_name || 'Unknown'}</span>
          <span class="reply-content">{replyingTo.content.slice(0, 100)}{replyingTo.content.length > 100 ? '...' : ''}</span>
        </div>
        <button class="cancel-reply" onclick={() => onCancelReply?.()} aria-label="Cancel reply" title="Cancel reply (Esc)">
          ×
        </button>
      </div>
    {/if}
    
    <div class="input-container">
      <textarea
        bind:this={inputRef}
        bind:value={messageText}
        placeholder={replyingTo ? "Type your reply..." : "Type a message..."}
        rows="1"
        disabled={isSending}
        onkeydown={handleKeydown}
        oninput={autoResize}
      />
      
      <button 
        class="send-btn"
        onclick={handleSend}
        disabled={!messageText.trim() || isSending}
        title="Send message (Enter)"
      >
        {#if isSending}
          <span class="spinner">⏳</span>
        {:else}
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M22 2L11 13M22 2l-7 20-4-9-9-4 20-7z" />
          </svg>
        {/if}
      </button>
    </div>
    
    <p class="hint">Press Enter to send, Shift+Enter for new line</p>
  {:else}
    <div class="disabled-state">
      Select a conversation to start messaging
    </div>
  {/if}
</div>

<style>
  .message-input {
    padding: 0.75rem 1.25rem;
    background: white;
    border-top: 1px solid #e5e7eb;
  }

  .input-container {
    display: flex;
    gap: 0.75rem;
    align-items: flex-end;
  }

  textarea {
    flex: 1;
    padding: 0.625rem 0.875rem;
    border: 1px solid #d1d5db;
    border-radius: 1.25rem;
    font-size: 0.9375rem;
    line-height: 1.4;
    resize: none;
    min-height: 2.5rem;
    max-height: 7.5rem;
    font-family: inherit;
    background: #f9fafb;
    transition: all 0.2s;
  }

  textarea:focus {
    outline: none;
    border-color: #0078d4;
    background: white;
    box-shadow: 0 0 0 3px rgba(0, 120, 212, 0.1);
  }

  textarea:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  textarea::placeholder {
    color: #9ca3af;
  }

  .send-btn {
    width: 2.5rem;
    height: 2.5rem;
    border-radius: 50%;
    border: none;
    background: #0078d4;
    color: white;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
    flex-shrink: 0;
  }

  .send-btn:hover:not(:disabled) {
    background: #006cbd;
    transform: scale(1.05);
  }

  .send-btn:disabled {
    background: #d1d5db;
    cursor: not-allowed;
  }

  .send-btn svg {
    width: 1.125rem;
    height: 1.125rem;
  }

  .spinner {
    font-size: 1rem;
  }

  .hint {
    margin: 0.5rem 0 0 0;
    font-size: 0.6875rem;
    color: #9ca3af;
    text-align: center;
  }

  .disabled-state {
    text-align: center;
    padding: 0.75rem;
    color: #9ca3af;
    font-size: 0.875rem;
    background: #f9fafb;
    border-radius: 0.5rem;
  }

  .reply-preview {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 0.75rem;
    margin-bottom: 0.5rem;
    background: #f0f9ff;
    border-left: 3px solid #0078d4;
    border-radius: 0.375rem;
    font-size: 0.8125rem;
  }

  .reply-info {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
  }

  .reply-label {
    color: #0078d4;
    font-weight: 500;
    font-size: 0.75rem;
  }

  .reply-content {
    color: #6b7280;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .cancel-reply {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 1.25rem;
    color: #9ca3af;
    padding: 0;
    width: 1.5rem;
    height: 1.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 0.25rem;
    flex-shrink: 0;
  }

  .cancel-reply:hover {
    background: rgba(0, 0, 0, 0.05);
    color: #6b7280;
  }
</style>
