import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export type Protocol = 'discord' | 'whatsapp' | 'signal';

export interface Conversation {
  id: string;
  protocol: Protocol;
  account_id: string;
  protocol_conversation_id: string;
  title?: string;
  participants: string[];
  last_message_id?: string;
  last_message_preview?: string;
  last_message_at?: number;
  unread_count: number;
  created_at: number;
  updated_at: number;
}

function createConversationsStore() {
  const { subscribe, set, update } = writable<Conversation[]>([]);

  return {
    subscribe,
    load: async () => {
      try {
        const convs = await invoke<Conversation[]>('get_conversations');
        set(convs);
      } catch (e) {
        console.error('Failed to load conversations:', e);
        set([]);
      }
    },
    loadForAccount: async (accountId: string) => {
      try {
        const convs = await invoke<Conversation[]>('get_conversations_for_account', { accountId });
        set(convs);
      } catch (e) {
        console.error('Failed to load conversations:', e);
        set([]);
      }
    },
    sync: async (accountId: string) => {
      try {
        const count = await invoke<number>('sync_conversations', { accountId });
        // Reload after sync
        const convs = await invoke<Conversation[]>('get_conversations_for_account', { accountId });
        set(convs);
        return count;
      } catch (e) {
        console.error('Failed to sync conversations:', e);
        throw e;
      }
    },
    markAsRead: async (conversationId: string) => {
      try {
        await invoke('mark_conversation_read', { conversationId });
        update(convs => 
          convs.map(conv => 
            conv.id === conversationId
              ? { ...conv, unread_count: 0 }
              : conv
          )
        );
      } catch (e) {
        console.error('Failed to mark conversation as read:', e);
        throw e;
      }
    },
    updateLastMessage: (conversationId: string, preview: string, timestamp: number) => {
      update(convs => 
        convs.map(conv => 
          conv.id === conversationId
            ? { 
                ...conv, 
                last_message_preview: preview, 
                last_message_at: timestamp,
                unread_count: conv.unread_count + 1
              }
            : conv
        )
      );
    },
    addConversation: (conversation: Conversation) => {
      update(convs => [conversation, ...convs]);
    }
  };
}

export const conversations = createConversationsStore();

// Selected conversation store for UI state
export const selectedConversation = writable<Conversation | null>(null);
