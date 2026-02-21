# Specification: Communications & Messaging Module

## Overview

This track will create a new module for secure, auditable, internal communication within the Skoola platform. This is a core feature for modern school portals.

## Functional Requirements

1.  **`conversations` Table:**
    *   Create a `conversations` table (`id`, `subject`, `created_at`).

2.  **`conversation_participants` Table:**
    *   Create a `conversation_participants` table (`conversation_id`, `user_id`).

3.  **`messages` Table:**
    *   Create a `messages` table (`id`, `conversation_id`, `sender_user_id`, `content`, `sent_at`, `read_at`).

4.  **Backend Services:**
    *   Create services for starting new conversations, sending messages, retrieving conversations for a user, and marking messages as read.

## Acceptance Criteria

*   A user can start a new conversation with one or more other users.
*   A user can send a message in a conversation.
*   A user can view all of their conversations.
*   A user can view all messages in a conversation.

## Out of Scope

*   This track does not include any UI/frontend for the messaging module.
*   This track does not include real-time messaging features (e.g., WebSockets).
