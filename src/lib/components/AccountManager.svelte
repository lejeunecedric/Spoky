<script lang="ts">
  import { accounts, type Account } from '$lib/stores/accounts';
  import { onMount } from 'svelte';

  let showAddAccount = $state(false);
  let botToken = $state('');
  let displayName = $state('');
  let isConnecting = $state(false);
  let error = $state('');

  onMount(() => {
    accounts.load();
  });

  async function handleAddAccount() {
    if (!botToken.trim()) {
      error = 'Bot token is required';
      return;
    }

    isConnecting = true;
    error = '';

    try {
      const account = await accounts.create('discord', botToken, displayName || undefined);
      
      // Auto-connect after creation
      await accounts.connect(account.id);
      
      // Reset form
      botToken = '';
      displayName = '';
      showAddAccount = false;
    } catch (e: any) {
      error = e.toString();
    } finally {
      isConnecting = false;
    }
  }

  function getStatusColor(status: string): string {
    switch (status) {
      case 'connected': return '#10b981';
      case 'connecting': return '#f59e0b';
      case 'error': return '#ef4444';
      default: return '#6b7280';
    }
  }

  function getStatusText(status: string): string {
    switch (status) {
      case 'connected': return 'Connected';
      case 'connecting': return 'Connecting...';
      case 'error': return 'Error';
      default: return 'Disconnected';
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
</script>

<div class="account-manager">
  <div class="header">
    <h3>Accounts</h3>
    <button class="add-btn" onclick={() => showAddAccount = !showAddAccount}>
      {showAddAccount ? 'Cancel' : '+ Add Account'}
    </button>
  </div>

  {#if showAddAccount}
    <div class="add-account-form">
      <h4>Add Discord Account</h4>
      
      <div class="form-group">
        <label for="bot-token">Bot Token *</label>
        <input
          id="bot-token"
          type="password"
          placeholder="Enter your Discord bot token..."
          bind:value={botToken}
        />
      </div>

      <div class="form-group">
        <label for="display-name">Display Name (optional)</label>
        <input
          id="display-name"
          type="text"
          placeholder="My Discord Bot"
          bind:value={displayName}
        />
      </div>

      {#if error}
        <div class="error">{error}</div>
      {/if}

      <button 
        class="submit-btn" 
        onclick={handleAddAccount}
        disabled={isConnecting}
      >
        {isConnecting ? 'Connecting...' : 'Add Account'}
      </button>

      <p class="help-text">
        Get your bot token from the 
        <a href="https://discord.com/developers/applications" target="_blank" rel="noopener">
          Discord Developer Portal
        </a>
      </p>
    </div>
  {/if}

  <div class="account-list">
    {#each $accounts as account (account.id)}
      <div class="account-item">
        <div class="account-info">
          <span class="protocol-icon">{getProtocolIcon(account.protocol)}</span>
          <div class="account-details">
            <span class="account-name">
              {account.display_name || account.protocol}
            </span>
            <span class="account-status" style="color: {getStatusColor(account.connection_status)}">
              ● {getStatusText(account.connection_status)}
            </span>
          </div>
        </div>
        
        <div class="account-actions">
          {#if account.connection_status === 'disconnected'}
            <button 
              class="action-btn connect" 
              onclick={() => accounts.connect(account.id)}
            >
              Connect
            </button>
          {:else if account.connection_status === 'connected'}
            <button 
              class="action-btn disconnect" 
              onclick={() => accounts.disconnect(account.id)}
            >
              Disconnect
            </button>
          {/if}
          
          <button 
            class="action-btn delete" 
            onclick={() => accounts.delete(account.id)}
          >
            🗑️
          </button>
        </div>
      </div>
    {:else}
      <p class="no-accounts">No accounts connected. Add one to get started!</p>
    {/each}
  </div>
</div>

<style>
  .account-manager {
    padding: 1rem;
    border-bottom: 1px solid #e5e7eb;
    background: #f9fafb;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  h3 {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
    color: #374151;
  }

  .add-btn {
    padding: 0.375rem 0.75rem;
    background: #0078d4;
    color: white;
    border: none;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    cursor: pointer;
    transition: background 0.2s;
  }

  .add-btn:hover {
    background: #006cbd;
  }

  .add-account-form {
    background: white;
    padding: 1rem;
    border-radius: 0.5rem;
    border: 1px solid #e5e7eb;
    margin-bottom: 1rem;
  }

  h4 {
    margin: 0 0 1rem 0;
    font-size: 0.9375rem;
    color: #374151;
  }

  .form-group {
    margin-bottom: 0.75rem;
  }

  label {
    display: block;
    font-size: 0.875rem;
    font-weight: 500;
    color: #374151;
    margin-bottom: 0.25rem;
  }

  input {
    width: 100%;
    padding: 0.5rem;
    border: 1px solid #d1d5db;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    box-sizing: border-box;
  }

  input:focus {
    outline: none;
    border-color: #0078d4;
    ring: 2px solid rgba(0, 120, 212, 0.2);
  }

  .error {
    color: #dc2626;
    font-size: 0.875rem;
    margin-bottom: 0.75rem;
  }

  .submit-btn {
    width: 100%;
    padding: 0.5rem;
    background: #10b981;
    color: white;
    border: none;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    cursor: pointer;
    transition: background 0.2s;
  }

  .submit-btn:hover:not(:disabled) {
    background: #059669;
  }

  .submit-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .help-text {
    margin: 0.75rem 0 0 0;
    font-size: 0.75rem;
    color: #6b7280;
  }

  .help-text a {
    color: #0078d4;
    text-decoration: none;
  }

  .help-text a:hover {
    text-decoration: underline;
  }

  .account-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .account-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem;
    background: white;
    border-radius: 0.375rem;
    border: 1px solid #e5e7eb;
  }

  .account-info {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .protocol-icon {
    font-size: 1.25rem;
  }

  .account-details {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }

  .account-name {
    font-weight: 500;
    font-size: 0.875rem;
    color: #111827;
  }

  .account-status {
    font-size: 0.75rem;
    font-weight: 500;
  }

  .account-actions {
    display: flex;
    gap: 0.5rem;
  }

  .action-btn {
    padding: 0.25rem 0.5rem;
    font-size: 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 0.25rem;
    background: white;
    cursor: pointer;
    transition: all 0.2s;
  }

  .action-btn:hover {
    background: #f3f4f6;
  }

  .action-btn.connect {
    color: #059669;
    border-color: #10b981;
  }

  .action-btn.connect:hover {
    background: #d1fae5;
  }

  .action-btn.disconnect {
    color: #dc2626;
    border-color: #ef4444;
  }

  .action-btn.disconnect:hover {
    background: #fee2e2;
  }

  .action-btn.delete {
    border-color: #e5e7eb;
  }

  .action-btn.delete:hover {
    background: #fee2e2;
    border-color: #ef4444;
  }

  .no-accounts {
    text-align: center;
    color: #6b7280;
    font-size: 0.875rem;
    padding: 1rem;
    margin: 0;
  }
</style>
