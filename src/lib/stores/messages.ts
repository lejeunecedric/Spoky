import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { conversations, selectedConversation } from './conversations';

export type ContentType = 'text' | 'image' | 'file';

export interface Message {
  id: string;
  conversation_id: string;
  protocol_message_id?: string;
  sender_id: string;
  sender_name?: string;
  content: string;
  content_type: ContentType;
  is_from_me: boolean;
  is_read: boolean;
  reply_to_message_id?: string;
  sent_at: number;
  received_at?: number;
  edited_at?: number;
}

interface MessageEvent {
  type: string;
  payload: {
    account_id: string;
    conversation_id: string;
    message: Message;
  };
}

function createMessagesStore() {
  const { subscribe, set, update } = writable<Message[]>([]);

  // Listen for new messages from backend
  listen('protocol:event', (event: MessageEvent) => {
    if (event.type === 'MessageReceived') {
      const msg = event.payload.message;
      
      // Add message to store if it matches current conversation
      selectedConversation.subscribe(currentConv => {
        if (currentConv && currentConv.id === msg.conversation_id) {
          update(msgs => [...msgs, msg]);
        }
      })();

      // Update conversation preview
      conversations.updateLastMessage(
        msg.conversation_id,
        msg.content.slice(0, 100),
        msg.sent_at
      );
    }
  }).catch(console.error);

  return {
    subscribe,
    load: async (conversationId: string) => {
      try {
        const msgs = await invoke<Message[]>('get_messages', { 
          conversationId,
          limit: 50 
        });
        set(msgs);
      } catch (e) {
        console.error('Failed to load messages:', e);
        set([]);
      }
    },
    loadMore: async (conversationId: string, before: number) => {
      try {
        const msgs = await invoke<Message[]>('get_messages', { 
          conversationId, 
          before,
          limit: 50 
        });
        update(existing => [...msgs, ...existing]);
        return msgs.length;
      } catch (e) {
        console.error('Failed to load more messages:', e);
        return 0;
      }
    },
    sync: async (conversationId: string) => {
      try {
        const count = await invoke<number>('sync_messages', { conversationId });
        // Reload after sync
        const msgs = await invoke<Message[]>('get_messages', { 
          conversationId,
          limit: 50 
        });
        set(msgs);
        return count;
      } catch (e) {
        console.error('Failed to sync messages:', e);
        throw e;
      }
    },
    send: async (conversationId: string, content: string, replyToMessageId?: string) => {
      try {
        const msg = await invoke<Message>('send_message', { 
          conversationId, 
          content,
          replyToMessageId
        });
        update(msgs => [...msgs, msg]);
        
        // Update conversation preview
        conversations.updateLastMessage(
          conversationId,
          content.slice(0, 100),
          msg.sent_at
        );
        
        return msg;
      } catch (e) {
        console.error('Failed to send message:', e);
        throw e;
      }
    },
    clear: () => {
      set([]);
    }
  };
}

export const messages = createMessagesStore();

// Helper to format timestamps
export function formatTime(timestamp: number): string {
  const date = new Date(timestamp);
  const now = new Date();
  const isToday = date.toDateString() === now.toDateString();
  
  if (isToday) {
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  }
  
  const yesterday = new Date(now);
  yesterday.setDate(yesterday.getDate() - 1);
  const isYesterday = date.toDateString() === yesterday.toDateString();
  
  if (isYesterday) {
    return 'Yesterday';
  }
  
  return date.toLocaleDateString([], { month: 'short', day: 'numeric' });
}

// Helper to format full date
export function formatFullDate(timestamp: number): string {
  const date = new Date(timestamp);
  return date.toLocaleString([], { 
    weekday: 'long',
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  });
}
