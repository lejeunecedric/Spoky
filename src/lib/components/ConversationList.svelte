<script lang="ts">
  import { conversations, selectedConversation, type Conversation } from '$lib/stores/conversations';
  import { accounts, type Account } from '$lib/stores/accounts';
  import { formatTime } from '$lib/stores/messages';
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  onMount(() => {
    conversations.load();
    accounts.load();
  });

  // New conversation modal state
  let showNewConversationModal = $state(false);
  let selectedAccount = $state<Account | null>(null);
  let userIdInput = $state('');
  let isCreating = $state(false);
  let createError = $state('');

  // Filter Discord accounts only (DMs only work for Discord in v1)
  let discordAccounts = $derived($accounts.filter(a => a.protocol === 'Discord'));

  async function handleCreateConversation() {
    if (!selectedAccount || !userIdInput.trim()) return;

    isCreating = true;
    createError = '';

    try {
      const conversation = await invoke<Conversation>('create_dm_conversation', {
        accountId: selectedAccount.id,
        userId: userIdInput.trim()
      });

      showNewConversationModal = false;
      userIdInput = '';
      selectedAccount = null;

      // Select the new conversation
      selectedConversation.set(conversation);

      // Refresh conversations list
      conversations.load();
    } catch (e) {
      createError = e instanceof Error ? e.message : 'Failed to create conversation';
      console.error('Failed to create conversation:', e);
    } finally {
      isCreating = false;
    }
  }

  function openNewConversationModal() {
    showNewConversationModal = true;
    // Pre-select first Discord account if available
    if (discordAccounts.length > 0 && !selectedAccount) {
      selectedAccount = discordAccounts[0];
    }
  }

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
    <div class="header-actions">
      <button 
        class="new-btn" 
        onclick={openNewConversationModal}
        title="New conversation"
      >
        +
      </button>
      <button 
        class="refresh-btn" 
        onclick={() => conversations.load()}
        title="Refresh conversations"
      >
        🔄
      </button>
    </div>
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

{#if showNewConversationModal}
  <div class="modal-backdrop" onclick={() => showNewConversationModal = false}>
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h3>New Direct Message</h3>
        <button class="close-btn" onclick={() => showNewConversationModal = false}>×</button>
      </div>

      {#if discordAccounts.length === 0}
        <div class="no-accounts">
          <p>Connect a Discord account first to start new conversations.</p>
        </div>
      {:else}
        <div class="form-group">
          <label for="account-select">Account</label>
          <select id="account-select" bind:value={selectedAccount}>
            {#each discordAccounts as account}
              <option value={account}>{account.display_name || account.protocol_id}</option>
            {/each}
          </select>
        </div>

        <div class="form-group">
          <label for="user-id-input">Discord User ID</label>
          <input
            id="user-id-input"
            type="text"
            bind:value={userIdInput}
            placeholder="Enter Discord user ID..."
            onkeydown={(e) => e.key === 'Enter' && handleCreateConversation()}
          />
          <p class="hint">
            Find a user's ID by right-clicking their name in Discord with Developer Mode enabled.
          </p>
        </div>

        {#if createError}
          <div class="error">{createError}</div>
        {/if}

        <div class="modal-actions">
          <button 
            class="secondary"
            onclick={() => showNewConversationModal = false}
          >
            Cancel
          </button>
          <button 
            class="primary"
            onclick={handleCreateConversation}
            disabled={!selectedAccount || !userIdInput.trim() || isCreating}
          >
            {isCreating ? 'Creating...' : 'Create'}
          </button>
        </div>
      {/if}
    </div>
  </div>
{/if}

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

  .header-actions {
    display: flex;
    gap: 0.5rem;
  }

  .new-btn {
    padding: 0.375rem 0.625rem;
    background: #0078d4;
    border: none;
    border-radius: 0.375rem;
    cursor: pointer;
    font-size: 1.125rem;
    color: white;
    line-height: 1;
    transition: all 0.2s;
    font-weight: 600;
  }

  .new-btn:hover {
    background: #006cbd;
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

  /* Modal Styles */
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 1rem;
  }

  .modal {
    background: white;
    border-radius: 0.75rem;
    padding: 1.5rem;
    min-width: 320px;
    max-width: 90vw;
    max-height: 90vh;
    overflow-y: auto;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.25rem;
  }

  .modal-header h3 {
    font-size: 1.125rem;
    font-weight: 600;
    color: #111827;
  }

  .close-btn {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 1.5rem;
    color: #9ca3af;
    padding: 0;
    width: 2rem;
    height: 2rem;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 0.375rem;
    transition: all 0.15s;
  }

  .close-btn:hover {
    background: #f3f4f6;
    color: #6b7280;
  }

  .form-group {
    margin-bottom: 1rem;
  }

  .form-group label {
    display: block;
    font-size: 0.875rem;
    font-weight: 500;
    color: #374151;
    margin-bottom: 0.375rem;
  }

  .form-group select,
  .form-group input {
    width: 100%;
    padding: 0.5rem 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 0.5rem;
    font-size: 0.9375rem;
    background: white;
    color: #111827;
  }

  .form-group select:focus,
  .form-group input:focus {
    outline: none;
    border-color: #0078d4;
    box-shadow: 0 0 0 3px rgba(0, 120, 212, 0.1);
  }

  .form-group .hint {
    font-size: 0.75rem;
    color: #6b7280;
    margin: 0.375rem 0 0 0;
  }

  .no-accounts {
    text-align: center;
    padding: 1.5rem 1rem;
    color: #6b7280;
  }

  .error {
    background: #fef2f2;
    color: #dc2626;
    padding: 0.75rem;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    margin-bottom: 1rem;
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    margin-top: 1.5rem;
  }

  .modal-actions button {
    padding: 0.5rem 1rem;
    border-radius: 0.5rem;
    font-size: 0.9375rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s;
  }

  .modal-actions button.secondary {
    background: white;
    border: 1px solid #d1d5db;
    color: #374151;
  }

  .modal-actions button.secondary:hover {
    background: #f9fafb;
  }

  .modal-actions button.primary {
    background: #0078d4;
    border: 1px solid #0078d4;
    color: white;
  }

  .modal-actions button.primary:hover:not(:disabled) {
    background: #006cbd;
    border-color: #006cbd;
  }

  .modal-actions button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
