# Specification: Advanced Timetabling & Resource Management

## Overview

This track extends the existing static timetable with advanced resource management capabilities. This will allow schools to manage the booking of rooms and equipment.

## Functional Requirements

1.  **`resources` Table:**
    *   Create a `resources` table (`id`, `resource_name`, `resource_type` (e.g., 'Equipment', 'Venue')).

2.  **`resource_bookings` Table:**
    *   Create a `resource_bookings` table (`id`, `resource_id`, `booked_by_user_id`, `start_time`, `end_time`, `related_event_id` (optional)).

3.  **Backend Services:**
    *   Create services for creating and managing resources.
    *   Create services for booking resources, checking for availability, and viewing bookings.

## Acceptance Criteria

*   A user can create a new resource (e.g., "Projector 1", "Auditorium").
*   A user can book a resource for a specific time slot.
*   The system prevents double-booking of resources.
*   A user can view all bookings for a specific resource.

## Out of Scope

*   This track does not include any UI/frontend for managing resources or bookings.
*   This track does not include integration with the existing timetable.
