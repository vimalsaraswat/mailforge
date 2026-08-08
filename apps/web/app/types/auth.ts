export interface CurrentUser {
  id: string
  email: string
  name: string
  picture?: string | null
  gmail_connected?: boolean
  gmail_connected_at?: string | null
}
