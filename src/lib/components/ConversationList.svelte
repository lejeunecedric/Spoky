<script lang="ts">
  import { conversations, selectedConversation, type Conversation } from '$lib/stores/conversations';
  import { formatTime } from '$lib/stores/messages';
  import { onMount } from 'svelte';

  onMount(() => {
    conversations.load();
  });

  function selectConversation(conv: Conversation) {
    selectedConversation.set(conv);
    if (conv.unread_count > 0) {
      conversations.markAsRead(conv.id);
    }
  }

  function getProtocolIcon(protocol: string): string {
    switch (protocol) {
      case 'discord': return '💬';
      case 'whatsapp': return '📱';
      case 'signal': return '🔒';
      default: return '💭';
    }
  }

  function getProtocolColor(protocol: string): string {
    switch (protocol) {
      case 'discord': return '#5865F2';
      case 'whatsapp': return '#25D366';
      case 'signal': return '#3A76F0';
      default: return '#6b7280';
    }
  }
</script>

<div class="conversation-list">
  <div class="header">
    <h3>Conversations</h3>
    <button 
      class="refresh-btn" 
      onclick={() => conversations.load()}
      title="Refresh conversations"
    >
      🔄
    </button>
  </div>

  <div class="list">
    {#each $conversations as conv (conv.id)}
      <button
        class="conversation-item"
        class:selected={$selectedConversation?.id === conv.id}
        class:unread={conv.unread_count > 0}
        onclick={() => selectConversation(conv)}
      >
        <div class="conversation-icon" style="background-color: {getProtocolColor(conv.protocol)}">
          {getProtocolIcon(conv.protocol)}
        </div>
        
        <div class="conversation-content">
          <div class="conversation-header">
            <span class="conversation-title">
              {conv.title || 'Unnamed Conversation'}
            </span>
            
            {#if conv.last_message_at}
              <span class="conversation-time">
                {formatTime(conv.last_message_at)}
              </span>
            {/if}
          </div>
          
          <div class="conversation-preview">
            {#if conv.last_message_preview}
              <span class="preview-text">
                {conv.last_message_preview}
              </span>
            {:else}
              <span class="preview-empty">No messages yet</span>
            {/if}
            
            {#if conv.unread_count > 0}
              <span class="unread-badge">{conv.unread_count}</span>
            {/if}
          </div>
        </div>
      </button>
    {:else}
      <div class="empty-state">
        <p>No conversations yet</p>
        <p class="empty-hint">Connect an account and sync to see your conversations</p>
      </div>
    {/each}
  </div>
</div>

<style>
  .conversation-list {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: white;
    border-right: 1px solid #e5e7eb;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    border-bottom: 1px solid #e5e7eb;
    background: #f9fafb;
  }

  h3 {
    margin: 0;
    font-size: 0.9375rem;
    font-weight: 600;
    color: #374151;
  }

  .refresh-btn {
    padding: 0.375rem;
    background: transparent;
    border: none;
    border-radius: 0.375rem;
    cursor: pointer;
    font-size: 1rem;
    opacity: 0.6;
    transition: all 0.2s;
  }

  .refresh-btn:hover {
    opacity: 1;
    background: #e5e7eb;
  }

  .list {
    flex: 1;
    overflow-y: auto;
    padding: 0.5rem;
  }

  .conversation-item {
    display: flex;
    align-items: flex-start;
    gap: 0.75rem;
    width: 100%;
    padding: 0.75rem;
    border: none;
    border-radius: 0.5rem;
    background: transparent;
    text-align: left;
    cursor: pointer;
    transition: background 0.15s;
  }

  .conversation-item:hover {
    background: #f3f4f6;
  }

  .conversation-item.selected {
    background: #dbeafe;
  }

  .conversation-item.unread {
    background: #eff6ff;
  }

  .conversation-item.unread:hover {
    background: #dbeafe;
  }

  .conversation-icon {
    width: 2.5rem;
    height: 2.5rem;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.125rem;
    flex-shrink: 0;
  }

  .conversation-content {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .conversation-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 0.5rem;
  }

  .conversation-title {
    font-weight: 500;
    font-size: 0.875rem;
    color: #111827;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .conversation-item.unread .conversation-title {
    font-weight: 600;
  }

  .conversation-time {
    font-size: 0.75rem;
    color: #6b7280;
    white-space: nowrap;
    flex-shrink: 0;
  }

  .conversation-preview {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 0.5rem;
  }

  .preview-text {
    font-size: 0.8125rem;
    color: #6b7280;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .conversation-item.unread .preview-text {
    color: #374151;
    font-weight: 500;
  }

  .preview-empty {
    font-size: 0.8125rem;
    color: #9ca3af;
    font-style: italic;
  }

  .unread-badge {
    background: #0078d4;
    color: white;
    font-size: 0.6875rem;
    font-weight: 600;
    min-width: 1.25rem;
    height: 1.25rem;
    border-radius: 0.625rem;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0 0.375rem;
    flex-shrink: 0;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 2rem 1rem;
    text-align: center;
    color: #6b7280;
  }

  .empty-state p {
    margin: 0;
  }

  .empty-hint {
    font-size: 0.8125rem;
    margin-top: 0.5rem !important;
    color: #9ca3af;
  }
</style>
