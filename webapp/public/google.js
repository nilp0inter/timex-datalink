// Google OAuth Client ID
const clientId = "430454772280-buh008fd6n9c0e312j147k1qfag9d5hm.apps.googleusercontent.com";
// Use a dynamic redirect URI based on current location
// This makes the code work across different environments (localhost, production, etc.)
const redirectUri = window.location.origin + window.location.pathname;
const scopes = [
    "https://www.googleapis.com/auth/calendar.readonly",
    "https://www.googleapis.com/auth/tasks.readonly",
    "https://www.googleapis.com/auth/contacts.readonly"
];

// Store Google data
let googleCalendarEvents = [];
let googleTasks = [];
let googleContacts = [];
let googleTaskLists = [];

// These PKCE functions are no longer used since we switched to implicit flow
// Left here for reference in case we need to switch back later
/*
async function generateCodeVerifier() {
    const array = new Uint32Array(56/2);
    window.crypto.getRandomValues(array);
    return Array.from(array, dec => ('0' + dec.toString(16)).substr(-2)).join('');
}

async function generateCodeChallenge(verifier) {
    const encoder = new TextEncoder();
    const data = encoder.encode(verifier);
    const digest = await window.crypto.subtle.digest('SHA-256', data);
    return btoa(String.fromCharCode(...new Uint8Array(digest)))
        .replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
}
*/

// For frontend-only OAuth, we'll use the implicit flow instead
// This avoids the need for a client secret
async function signInWithGoogle() {
    log("Starting Google authentication...", false);
    
    // Use implicit grant flow (response_type=token)
    const authUrl = new URL("https://accounts.google.com/o/oauth2/v2/auth");
    authUrl.searchParams.set('client_id', clientId);
    authUrl.searchParams.set('redirect_uri', redirectUri);
    authUrl.searchParams.set('response_type', 'token');
    authUrl.searchParams.set('scope', scopes.join(' '));
    
    console.log("Auth URL:", authUrl.toString());
    window.location = authUrl.toString();
}

// Check for access token in URL fragment after implicit OAuth redirect
function checkAuthTokenInUrl() {
    // In implicit flow, token comes back in URL fragment
    if (window.location.hash) {
        log("Processing authentication response...", false);
        
        // Parse the URL hash fragment
        const params = new URLSearchParams(window.location.hash.substring(1));
        const accessToken = params.get('access_token');
        const expiresIn = params.get('expires_in');
        
        if (accessToken) {
            console.log("Access token found in URL");
            
            // Store the token
            sessionStorage.setItem('access_token', accessToken);
            sessionStorage.setItem('token_expiry', Date.now() + (parseInt(expiresIn) * 1000));
            
            // Clean up the URL
            window.history.replaceState({}, document.title, window.location.pathname);
            
            log('Successfully signed in with Google', false);
            
            // Update UI first
            updateAuthUI(true);
            
            // Initialize date inputs for calendar after successful login
            initCalendarDateInputs();
            
            // Load task lists
            loadTaskLists();
            
            return true;
        }
    }
    
    // Check if we already have a valid token
    if (isTokenValid()) {
        log(`Using existing valid token`, false);
        updateAuthUI(true);
        
        // Initialize calendar date inputs
        initCalendarDateInputs();
        
        // Load task lists
        loadTaskLists();
        
        return true;
    }
    
    return false;
}

// Sign out
function signOut() {
    // Clear session storage
    sessionStorage.removeItem('access_token');
    sessionStorage.removeItem('token_expiry');
    sessionStorage.removeItem('code_verifier');
    
    // Redirect to Google's revocation endpoint to fully sign out
    const revokeUrl = "https://accounts.google.com/o/oauth2/revoke?token=" + sessionStorage.getItem('access_token');
    
    // Create a hidden iframe to trigger Google logout without navigating away
    const iframe = document.createElement('iframe');
    iframe.style.display = 'none';
    iframe.src = "https://accounts.google.com/logout";
    document.body.appendChild(iframe);
    
    // Remove iframe after logout completes and update UI
    setTimeout(() => {
        document.body.removeChild(iframe);
        updateAuthUI(false);
        log('Signed out from Google');
    }, 1000);
}

// Update UI based on authentication state
function updateAuthUI(isAuthenticated) {
    document.getElementById('signInButton').style.display = isAuthenticated ? 'none' : 'block';
    document.getElementById('signOutButton').style.display = isAuthenticated ? 'block' : 'none';
    document.getElementById('googleCalendarSection').style.display = isAuthenticated ? 'block' : 'none';
    document.getElementById('googleTasksSection').style.display = isAuthenticated ? 'block' : 'none';
    document.getElementById('googleContactsSection').style.display = isAuthenticated ? 'block' : 'none';
}

// Check if the access token is valid
function isTokenValid() {
    const token = sessionStorage.getItem('access_token');
    const expiry = sessionStorage.getItem('token_expiry');
    
    if (!token || !expiry) return false;
    
    return Date.now() < parseInt(expiry);
}

// Initialize calendar date inputs with current week
function initCalendarDateInputs() {
    const today = new Date();
    const startDate = new Date(today);
    startDate.setDate(today.getDate() - today.getDay()); // Start of current week (Sunday)
    
    const endDate = new Date(startDate);
    endDate.setDate(startDate.getDate() + 6); // End of current week (Saturday)
    
    document.getElementById('calendarDateStart').value = formatDateInput(startDate);
    document.getElementById('calendarDateEnd').value = formatDateInput(endDate);
}

// Format date for date input (YYYY-MM-DD)
function formatDateInput(date) {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    return `${year}-${month}-${day}`;
}

// Format date to datetime-local input format (YYYY-MM-DDThh:mm)
function formatDatetimeLocal(date) {
    return date.toISOString().substring(0, 16);
}

// Format date and time for display
function formatDateTime(dateStr, timeStr) {
    const date = new Date(dateStr);
    
    let formattedDate = `${date.toLocaleDateString(undefined, {
        weekday: 'short',
        month: 'short',
        day: 'numeric'
    })}`;
    
    if (timeStr) {
        formattedDate += ` ${timeStr}`;
    }
    
    return formattedDate;
}

// Determine phone type code
function determinePhoneType(type) {
    if (!type) return 'O'; // Other
    
    const typeStr = type.toLowerCase();
    if (typeStr.includes('home')) return 'H';
    if (typeStr.includes('work')) return 'W';
    if (typeStr.includes('mobile') || typeStr.includes('cell')) return 'C';
    if (typeStr.includes('fax')) return 'F';
    return 'O'; // Other
}

// Extract clean phone number
function cleanPhoneNumber(number) {
    if (!number) return '';
    // Remove everything except digits
    return number.replace(/[^\d]/g, '');
}

// Get priority based on position (1-5)
function getTaskPriority(index, total) {
    // Distribute tasks evenly across priorities 1-5
    if (total <= 5) {
        return index + 1;
    } else {
        return Math.min(Math.floor((index / total) * 5) + 1, 5);
    }
}

// Load the user's Google Calendar events
async function loadCalendarEvents() {
    if (!isTokenValid()) {
        log('Not authenticated with Google Calendar', true);
        return;
    }
    
    try {
        const token = sessionStorage.getItem('access_token');
        const startDate = document.getElementById('calendarDateStart').value;
        const endDate = document.getElementById('calendarDateEnd').value;
        
        if (!startDate || !endDate) {
            log('Please select a date range', true);
            return;
        }
        
        // Add time to dates for API query
        const timeMin = `${startDate}T00:00:00Z`;
        const timeMax = `${endDate}T23:59:59Z`;
        
        log(`Loading calendar events from ${startDate} to ${endDate}...`);
        
        const response = await fetch(
            `https://www.googleapis.com/calendar/v3/calendars/primary/events?timeMin=${timeMin}&timeMax=${timeMax}&maxResults=50&orderBy=startTime&singleEvents=true`, 
            {
                headers: { 'Authorization': `Bearer ${token}` }
            }
        );
        
        const data = await response.json();
        
        if (data.error) {
            throw new Error(data.error.message || 'Failed to fetch calendar events');
        }
        
        if (!data.items || data.items.length === 0) {
            log('No calendar events found in the selected period');
            return;
        }
        
        log(`Found ${data.items.length} calendar events`);
        
        // Process events data
        googleCalendarEvents = data.items.map(event => {
            let startTime, formattedTime;
            
            if (event.start.dateTime) {
                // This is a timed event
                startTime = new Date(event.start.dateTime);
                const hours = startTime.getHours();
                const minutes = startTime.getMinutes();
                formattedTime = `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}`;
            } else {
                // This is an all-day event
                startTime = new Date(`${event.start.date}T12:00:00`);
                formattedTime = 'All day';
            }
            
            return {
                id: event.id,
                title: event.summary || 'Untitled Event',
                time: formattedTime,
                date: event.start.dateTime || event.start.date,
                allDay: !event.start.dateTime,
                watchDate: formatDatetimeLocal(startTime),
                watchMessage: (event.summary || 'Untitled').substring(0, 12) // Limit to 12 chars
            };
        });
        
        // Display events
        displayCalendarEvents();
        
    } catch (error) {
        log(`Error loading calendar events: ${error.message}`, true);
    }
}

// Display calendar events in the selection list
function displayCalendarEvents() {
    const eventsContainer = document.getElementById('calendarEventsContainer');
    const eventsList = document.getElementById('calendarEventsList');
    
    // Clear previous content
    eventsList.innerHTML = '';
    
    // Create HTML for each event
    googleCalendarEvents.forEach(event => {
        const eventElement = document.createElement('div');
        eventElement.className = 'google-item';
        
        const checkbox = document.createElement('input');
        checkbox.type = 'checkbox';
        checkbox.className = 'google-item-checkbox';
        checkbox.dataset.id = event.id;
        checkbox.checked = true; // Default to checked
        
        const detailsDiv = document.createElement('div');
        detailsDiv.className = 'google-item-details';
        
        const titleDiv = document.createElement('div');
        titleDiv.className = 'google-item-title';
        titleDiv.textContent = event.title;
        
        const subtitleDiv = document.createElement('div');
        subtitleDiv.className = 'google-item-subtitle';
        subtitleDiv.textContent = `${formatDateTime(event.date, event.time)}`;
        
        detailsDiv.appendChild(titleDiv);
        detailsDiv.appendChild(subtitleDiv);
        
        eventElement.appendChild(checkbox);
        eventElement.appendChild(detailsDiv);
        
        eventsList.appendChild(eventElement);
    });
    
    // Show the events container
    eventsContainer.style.display = 'block';
}

// Import selected calendar events to appointments
function importSelectedCalendarEvents() {
    const checkboxes = document.querySelectorAll('#calendarEventsList .google-item-checkbox:checked');
    
    if (checkboxes.length === 0) {
        log('No events selected to import');
        return;
    }
    
    log(`Importing ${checkboxes.length} calendar events to appointments...`);
    
    // Get selected event IDs
    const selectedIds = Array.from(checkboxes).map(cb => cb.dataset.id);
    
    // Filter events that were selected
    const selectedEvents = googleCalendarEvents.filter(event => selectedIds.includes(event.id));
    
    // Convert events to appointments and add to the watch
    const appointments = selectedEvents.map(event => ({
        date: event.watchDate,
        message: event.watchMessage
    }));
    
    // Clear existing appointments
    const appointmentsList = document.getElementById('appointmentsList');
    appointmentsList.innerHTML = '';
    
    // Add new appointments from calendar
    appointments.forEach(appointment => {
        appointmentsList.appendChild(createAppointmentRow(appointment));
    });
    
    // Save the imported appointments to localStorage
    saveFormData();
    
    log(`Successfully imported ${appointments.length} events to appointments`);
}

// Load task lists
async function loadTaskLists() {
    if (!isTokenValid()) {
        log('Not authenticated with Google Tasks', true);
        return;
    }
    
    try {
        const token = sessionStorage.getItem('access_token');
        
        const response = await fetch(
            'https://tasks.googleapis.com/tasks/v1/users/@me/lists',
            {
                headers: { 'Authorization': `Bearer ${token}` }
            }
        );
        
        const data = await response.json();
        
        if (data.error) {
            throw new Error(data.error.message || 'Failed to fetch task lists');
        }
        
        if (!data.items || data.items.length === 0) {
            log('No task lists found');
            return;
        }
        
        // Store task lists
        googleTaskLists = data.items;
        
        // Update task list dropdown
        const taskListSelect = document.getElementById('taskListSelect');
        taskListSelect.innerHTML = '';
        
        googleTaskLists.forEach(list => {
            const option = document.createElement('option');
            option.value = list.id;
            option.textContent = list.title;
            taskListSelect.appendChild(option);
        });
        
    } catch (error) {
        log(`Error loading task lists: ${error.message}`, true);
    }
}

// Load tasks from the selected list
async function loadTasks() {
    if (!isTokenValid()) {
        log('Not authenticated with Google Tasks', true);
        return;
    }
    
    const taskListSelect = document.getElementById('taskListSelect');
    const taskListId = taskListSelect.value;
    
    if (!taskListId) {
        log('Please select a task list', true);
        return;
    }
    
    try {
        const token = sessionStorage.getItem('access_token');
        
        log(`Loading tasks from list: ${taskListSelect.options[taskListSelect.selectedIndex].text}...`);
        
        const response = await fetch(
            `https://tasks.googleapis.com/tasks/v1/lists/${taskListId}/tasks?maxResults=100`,
            {
                headers: { 'Authorization': `Bearer ${token}` }
            }
        );
        
        const data = await response.json();
        
        if (data.error) {
            throw new Error(data.error.message || 'Failed to fetch tasks');
        }
        
        if (!data.items || data.items.length === 0) {
            log('No tasks found in the selected list');
            return;
        }
        
        log(`Found ${data.items.length} tasks`);
        
        // Process tasks data
        googleTasks = data.items
            .filter(task => task.title) // Only tasks with titles
            .map((task, index, array) => ({
                id: task.id,
                title: task.title,
                notes: task.notes,
                completed: task.status === 'completed',
                priority: getTaskPriority(index, array.length),
                watchEntry: task.title.substring(0, 12), // Limit to 12 chars
                watchPriority: getTaskPriority(index, array.length)
            }));
        
        // Display tasks
        displayTasks();
        
    } catch (error) {
        log(`Error loading tasks: ${error.message}`, true);
    }
}

// Display tasks in the selection list
function displayTasks() {
    const tasksContainer = document.getElementById('tasksContainer');
    const tasksList = document.getElementById('tasksList');
    
    // Clear previous content
    tasksList.innerHTML = '';
    
    // Create HTML for each task
    googleTasks.forEach(task => {
        const taskElement = document.createElement('div');
        taskElement.className = 'google-item';
        if (task.completed) {
            taskElement.style.opacity = '0.5';
        }
        
        const checkbox = document.createElement('input');
        checkbox.type = 'checkbox';
        checkbox.className = 'google-item-checkbox';
        checkbox.dataset.id = task.id;
        checkbox.checked = !task.completed; // Only check non-completed tasks by default
        
        const detailsDiv = document.createElement('div');
        detailsDiv.className = 'google-item-details';
        
        const titleDiv = document.createElement('div');
        titleDiv.className = 'google-item-title';
        titleDiv.textContent = task.title;
        
        const subtitleDiv = document.createElement('div');
        subtitleDiv.className = 'google-item-subtitle';
        subtitleDiv.textContent = `Priority: ${task.priority}${task.completed ? ' • Completed' : ''}`;
        
        detailsDiv.appendChild(titleDiv);
        detailsDiv.appendChild(subtitleDiv);
        
        taskElement.appendChild(checkbox);
        taskElement.appendChild(detailsDiv);
        
        tasksList.appendChild(taskElement);
    });
    
    // Show the tasks container
    tasksContainer.style.display = 'block';
    
    // Add filter functionality
    const tasksSearchInput = document.getElementById('tasksSearchInput');
    tasksSearchInput.addEventListener('input', filterTasks);
    tasksSearchInput.value = ''; // Clear the input
}

// Filter tasks based on search input
function filterTasks() {
    const searchText = document.getElementById('tasksSearchInput').value.toLowerCase();
    const taskItems = document.querySelectorAll('#tasksList .google-item');
    
    taskItems.forEach(item => {
        const title = item.querySelector('.google-item-title').textContent.toLowerCase();
        
        if (title.includes(searchText)) {
            item.style.display = 'flex';
        } else {
            item.style.display = 'none';
        }
    });
}

// Import selected tasks to to-do list
function importSelectedTasks() {
    const checkboxes = document.querySelectorAll('#tasksList .google-item-checkbox:checked');
    
    if (checkboxes.length === 0) {
        log('No tasks selected to import');
        return;
    }
    
    log(`Importing ${checkboxes.length} tasks to to-do list...`);
    
    // Get selected task IDs
    const selectedIds = Array.from(checkboxes).map(cb => cb.dataset.id);
    
    // Filter tasks that were selected
    const selectedTasks = googleTasks.filter(task => selectedIds.includes(task.id));
    
    // Convert tasks to list items and add to the watch
    const listItems = selectedTasks.map(task => ({
        entry: task.watchEntry,
        priority: task.watchPriority
    }));
    
    // Clear existing list items
    const listsList = document.getElementById('listsList');
    listsList.innerHTML = '';
    
    // Add new list items from tasks
    listItems.forEach(item => {
        listsList.appendChild(createListRow(item));
    });
    
    // Save the imported tasks to localStorage
    saveFormData();
    
    log(`Successfully imported ${listItems.length} tasks to to-do list`);
}

// Load and search contacts
async function loadContacts() {
    if (!isTokenValid()) {
        log('Not authenticated with Google Contacts', true);
        return;
    }
    
    const searchQuery = document.getElementById('contactsSearchInput').value.trim();
    
    try {
        const token = sessionStorage.getItem('access_token');
        
        log(`Searching contacts for: ${searchQuery || 'all contacts'}...`);
        
        let url = 'https://people.googleapis.com/v1/people/me/connections?personFields=names,phoneNumbers&pageSize=50';
        
        if (searchQuery) {
            // Use searchContacts API if there's a search query
            url = `https://people.googleapis.com/v1/people:searchContacts?query=${encodeURIComponent(searchQuery)}&readMask=names,phoneNumbers`;
        }
        
        const response = await fetch(url, {
            headers: { 'Authorization': `Bearer ${token}` }
        });
        
        const data = await response.json();
        
        if (data.error) {
            throw new Error(data.error.message || 'Failed to fetch contacts');
        }
        
        // The response structure differs between the two APIs
        const connections = searchQuery ? data.results : data.connections;
        
        if (!connections || connections.length === 0) {
            log('No contacts found matching your search');
            return;
        }
        
        log(`Found ${connections.length} contacts`);
        
        // Process contacts data
        googleContacts = connections
            .filter(person => {
                // Only include contacts with both a name and at least one phone number
                return person.names && person.names.length > 0 && 
                       person.phoneNumbers && person.phoneNumbers.length > 0;
            })
            .map(person => {
                const name = person.names[0].displayName || 'Unknown';
                const primaryPhone = person.phoneNumbers[0];
                
                return {
                    resourceName: person.resourceName,
                    name: name,
                    phoneNumber: primaryPhone.value,
                    phoneType: primaryPhone.type || 'other',
                    watchName: name.substring(0, 12), // Limit to 12 chars
                    watchNumber: cleanPhoneNumber(primaryPhone.value),
                    watchType: determinePhoneType(primaryPhone.type)
                };
            });
        
        // Display contacts
        displayContacts();
        
    } catch (error) {
        log(`Error loading contacts: ${error.message}`, true);
    }
}

// Display contacts in the selection list
function displayContacts() {
    const contactsContainer = document.getElementById('contactsContainer');
    const contactsList = document.getElementById('contactsList');
    
    // Clear previous content
    contactsList.innerHTML = '';
    
    // Create HTML for each contact
    googleContacts.forEach(contact => {
        const contactElement = document.createElement('div');
        contactElement.className = 'google-item';
        
        const checkbox = document.createElement('input');
        checkbox.type = 'checkbox';
        checkbox.className = 'google-item-checkbox';
        checkbox.dataset.id = contact.resourceName;
        checkbox.checked = true; // Default to checked
        
        const detailsDiv = document.createElement('div');
        detailsDiv.className = 'google-item-details';
        
        const titleDiv = document.createElement('div');
        titleDiv.className = 'google-item-title';
        titleDiv.textContent = contact.name;
        
        const subtitleDiv = document.createElement('div');
        subtitleDiv.className = 'google-item-subtitle';
        subtitleDiv.textContent = `${contact.phoneNumber} (${contact.phoneType})`;
        
        detailsDiv.appendChild(titleDiv);
        detailsDiv.appendChild(subtitleDiv);
        
        contactElement.appendChild(checkbox);
        contactElement.appendChild(detailsDiv);
        
        contactsList.appendChild(contactElement);
    });
    
    // Show the contacts container
    contactsContainer.style.display = 'block';
}

// Import selected contacts to phone numbers
function importSelectedContacts() {
    const checkboxes = document.querySelectorAll('#contactsList .google-item-checkbox:checked');
    
    if (checkboxes.length === 0) {
        log('No contacts selected to import');
        return;
    }
    
    log(`Importing ${checkboxes.length} contacts to phone numbers...`);
    
    // Get selected contact IDs
    const selectedIds = Array.from(checkboxes).map(cb => cb.dataset.id);
    
    // Filter contacts that were selected
    const selectedContacts = googleContacts.filter(contact => selectedIds.includes(contact.resourceName));
    
    // Convert contacts to phone entries and add to the watch
    const phoneEntries = selectedContacts.map(contact => ({
        name: contact.watchName,
        number: contact.watchNumber,
        type: contact.watchType
    }));
    
    // Clear existing phone numbers
    const phoneNumbersList = document.getElementById('phoneNumbersList');
    phoneNumbersList.innerHTML = '';
    
    // Add new phone numbers from contacts
    phoneEntries.forEach(phone => {
        phoneNumbersList.appendChild(createPhoneNumberRow(phone));
    });
    
    // Save the imported contacts to localStorage
    saveFormData();
    
    log(`Successfully imported ${phoneEntries.length} contacts to phone numbers`);
}

// The old code-based auth method is no longer used
// This is replaced by checkAuthTokenInUrl() which handles the implicit flow

// Select/deselect all helpers
function setupSelectionHelpers() {
    // Calendar events
    document.getElementById('selectAllCalendarEvents').addEventListener('click', () => {
        document.querySelectorAll('#calendarEventsList .google-item-checkbox').forEach(cb => cb.checked = true);
    });
    
    document.getElementById('deselectAllCalendarEvents').addEventListener('click', () => {
        document.querySelectorAll('#calendarEventsList .google-item-checkbox').forEach(cb => cb.checked = false);
    });
    
    // Tasks
    document.getElementById('selectAllTasks').addEventListener('click', () => {
        document.querySelectorAll('#tasksList .google-item-checkbox:not([style*="display: none"])').forEach(cb => cb.checked = true);
    });
    
    document.getElementById('deselectAllTasks').addEventListener('click', () => {
        document.querySelectorAll('#tasksList .google-item-checkbox:not([style*="display: none"])').forEach(cb => cb.checked = false);
    });
    
    // Contacts
    document.getElementById('selectAllContacts').addEventListener('click', () => {
        document.querySelectorAll('#contactsList .google-item-checkbox').forEach(cb => cb.checked = true);
    });
    
    document.getElementById('deselectAllContacts').addEventListener('click', () => {
        document.querySelectorAll('#contactsList .google-item-checkbox').forEach(cb => cb.checked = false);
    });
}

// Initialize Google API integration
function initGoogleIntegration() {
    // First check if we're handling an auth redirect with token
    const isHandlingAuth = checkAuthTokenInUrl();
    
    if (isHandlingAuth) {
        console.log("Handling authentication redirect");
        log("Handling Google authentication redirect...", false);
    }
    
    // Auth buttons
    document.getElementById('signInButton').addEventListener('click', signInWithGoogle);
    document.getElementById('signOutButton').addEventListener('click', signOut);
    
    // Calendar buttons
    document.getElementById('loadCalendarButton').addEventListener('click', loadCalendarEvents);
    document.getElementById('importCalendarEvents').addEventListener('click', importSelectedCalendarEvents);
    
    // Tasks buttons
    document.getElementById('loadTasksButton').addEventListener('click', loadTasks);
    document.getElementById('importTasks').addEventListener('click', importSelectedTasks);
    
    // Contacts buttons
    document.getElementById('loadContactsButton').addEventListener('click', loadContacts);
    document.getElementById('importContacts').addEventListener('click', importSelectedContacts);
    
    // Setup select/deselect all helpers
    setupSelectionHelpers();
}

// Add Google integration initialization to page load
// Use DOMContentLoaded to ensure this runs as early as possible
document.addEventListener('DOMContentLoaded', initGoogleIntegration);

