import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export type Protocol = 'discord' | 'whatsapp' | 'signal';
export type ConnectionStatus = 'disconnected' | 'connecting' | 'connected' | 'error';

export interface Account {
  id: string;
  protocol: Protocol;
  display_name?: string;
  connection_status: ConnectionStatus;
  created_at: number;
  updated_at: number;
}

interface AccountEvent {
  type: string;
  payload: {
    account_id: string;
    protocol: Protocol;
    status: ConnectionStatus;
    message?: string;
  };
}

function createAccountsStore() {
  const { subscribe, set, update } = writable<Account[]>([]);

  // Listen for connection status updates from backend
  listen('protocol:event', (event: AccountEvent) => {
    if (event.type === 'ConnectionChanged') {
      update(accounts => 
        accounts.map(acc => 
          acc.id === event.payload.account_id
            ? { ...acc, connection_status: event.payload.status }
            : acc
        )
      );
    }
  }).catch(console.error);

  return {
    subscribe,
    load: async () => {
      try {
        const accounts = await invoke<Account[]>('get_accounts');
        set(accounts);
      } catch (e) {
        console.error('Failed to load accounts:', e);
        set([]);
      }
    },
    create: async (protocol: string, credentials: string, displayName?: string) => {
      try {
        const account = await invoke<Account>('create_account', { 
          protocol, 
          credentials, 
          displayName 
        });
        update(accounts => [...accounts, account]);
        return account;
      } catch (e) {
        console.error('Failed to create account:', e);
        throw e;
      }
    },
    delete: async (accountId: string) => {
      try {
        await invoke('delete_account', { accountId });
        update(accounts => accounts.filter(acc => acc.id !== accountId));
      } catch (e) {
        console.error('Failed to delete account:', e);
        throw e;
      }
    },
    connect: async (accountId: string) => {
      try {
        await invoke('connect_discord_account', { accountId });
        update(accounts => 
          accounts.map(acc => 
            acc.id === accountId
              ? { ...acc, connection_status: 'connecting' as ConnectionStatus }
              : acc
          )
        );
      } catch (e) {
        console.error('Failed to connect account:', e);
        throw e;
      }
    },
    disconnect: async (accountId: string) => {
      try {
        await invoke('disconnect_account', { accountId });
        update(accounts => 
          accounts.map(acc => 
            acc.id === accountId
              ? { ...acc, connection_status: 'disconnected' as ConnectionStatus }
              : acc
          )
        );
      } catch (e) {
        console.error('Failed to disconnect account:', e);
        throw e;
      }
    }
  };
}

export const accounts = createAccountsStore();
