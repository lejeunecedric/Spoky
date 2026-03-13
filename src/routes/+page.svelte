<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { accounts } from '$lib/stores/accounts';
  import { conversations } from '$lib/stores/conversations';
  import { messages } from '$lib/stores/messages';
  
  import AccountManager from '$lib/components/AccountManager.svelte';
  import ConversationList from '$lib/components/ConversationList.svelte';
  import ConversationView from '$lib/components/ConversationView.svelte';
  import MessageInput from '$lib/components/MessageInput.svelte';

  // Listen for protocol events to update UI in real-time
  onMount(() => {
    const unlisten = listen('protocol:event', (event: any) => {
      console.log('Protocol event received:', event);
      
      // Handle different event types
      switch (event.payload?.type) {
        case 'MessageReceived':
          const msg = event.payload.payload?.message;
          if (msg) {
            // Update conversations store with new message preview
            conversations.updateLastMessage(
              msg.conversation_id,
              msg.content.slice(0, 100),
              msg.sent_at
            );
          }
          break;
          
        case 'ConversationUpdated':
          // Reload conversations to show new/updated conversation
          conversations.load();
          break;
          
        case 'ConnectionChanged':
          // Reload accounts to get updated connection status
          accounts.load();
          break;
      }
    });

    // Load initial data
    accounts.load();
    conversations.load();

    return () => {
      unlisten.then(fn => fn());
    };
  });
</script>

<div class="app-container">
  <!-- Left Sidebar -->
  <aside class="sidebar">
    <div class="logo">
      <h1>Spoky</h1>
    </div>
    <div class="sidebar-content">
      <AccountManager />
      <div class="conversation-list-wrapper">
        <ConversationList />
      </div>
    </div>
  </aside>

  <!-- Main Content Area -->
  <main class="main-content">
    <ConversationView />
    <MessageInput />
  </main>
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
    background: #f3f4f6;
    height: 100vh;
    overflow: hidden;
  }

  :global(*) {
    box-sizing: border-box;
  }

  .app-container {
    display: flex;
    height: 100vh;
    width: 100vw;
  }

  .sidebar {
    width: 320px;
    min-width: 280px;
    max-width: 400px;
    display: flex;
    flex-direction: column;
    background: white;
    border-right: 1px solid #e5e7eb;
    box-shadow: 2px 0 4px rgba(0, 0, 0, 0.05);
  }

  .logo {
    padding: 1rem 1.25rem;
    border-bottom: 1px solid #e5e7eb;
    background: linear-gradient(135deg, #0078d4 0%, #106ebe 100%);
  }

  .logo h1 {
    margin: 0;
    font-size: 1.5rem;
    font-weight: 700;
    color: white;
    letter-spacing: -0.025em;
  }

  .sidebar-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .conversation-list-wrapper {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .main-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    background: #f9fafb;
    min-width: 0;
  }

  /* Responsive adjustments */
  @media (max-width: 768px) {
    .sidebar {
      width: 100%;
      max-width: none;
    }

    .main-content {
      display: none;
    }
  }
</style>
