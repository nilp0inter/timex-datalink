// Web Serial port
let port = null;

// Default example data
const defaultAlarms = [
    { number: 1, audible: true, hour: 9, minute: 0, message: "Wake up" },
    { number: 2, audible: true, hour: 9, minute: 5, message: "For real" },
    { number: 3, audible: false, hour: 9, minute: 10, message: "Get up" },
    { number: 4, audible: true, hour: 9, minute: 15, message: "Or not" },
    { number: 5, audible: false, hour: 11, minute: 30, message: "Told you" }
];

const defaultAppointments = [
    { date: "2023-10-31T19:00", message: "Scare the neighbors" },
    { date: "2023-11-24T17:00", message: "Feed the neighbors" },
    { date: "2023-12-25T14:00", message: "Spoil the neighbors" }
];

const defaultAnniversaries = [
    { date: "1985-07-03", message: "Release of Back to the Future" },
    { date: "1968-04-06", message: "Release of 2001" }
];

const defaultPhoneNumbers = [
    { name: "Marty McFly", number: "1112223333", type: "H" },
    { name: "Doc Brown", number: "4445556666", type: "C" }
];

const defaultLists = [
    { entry: "Muffler bearings", priority: 2 },
    { entry: "Headlight fluid", priority: 4 }
];

// Function to log messages to the console
function log(message, isError = false) {
    const logElem = document.getElementById('log');
    const entry = document.createElement('div');
    entry.textContent = message;
    if (isError) {
        entry.classList.add('error');
    }
    logElem.appendChild(entry);
    logElem.scrollTop = logElem.scrollHeight;
    console.log(message);
}

// Function to update the status message
function updateStatus(message, isError = false, isSuccess = false) {
    const statusElem = document.getElementById('status');
    statusElem.textContent = message;
    
    statusElem.classList.remove('error', 'success');
    if (isError) {
        statusElem.classList.add('error');
    } else if (isSuccess) {
        statusElem.classList.add('success');
    }
}

// Function to connect to the serial port
async function connectToSerialPort() {
    try {
        port = await navigator.serial.requestPort();
        await port.open({ baudRate: 9600 });
        
        updateStatus('Connected to serial port', false, true);
        log('Serial port connected');
        
        document.getElementById('connectButton').disabled = true;
        document.getElementById('sendDataButton').disabled = false;
        document.getElementById('disconnectButton').disabled = false;
        
    } catch (error) {
        updateStatus('Failed to connect to serial port', true);
        log(`Error connecting to serial port: ${error.message}`, true);
        console.error(error);
    }
}

// Function to disconnect from the serial port
async function disconnectFromSerialPort() {
    if (port) {
        try {
            await port.close();
            port = null;
            
            updateStatus('Disconnected from serial port');
            log('Serial port disconnected');
            
            document.getElementById('connectButton').disabled = false;
            document.getElementById('sendDataButton').disabled = true;
            document.getElementById('disconnectButton').disabled = true;
            
        } catch (error) {
            updateStatus('Failed to disconnect from serial port', true);
            log(`Error disconnecting from serial port: ${error.message}`, true);
            console.error(error);
        }
    }
}

// Function to set the current time in the datetime-local input (in UTC)
function setCurrentTime() {
    const now = new Date();
    // Format: YYYY-MM-DDThh:mm (in UTC)
    const year = now.getUTCFullYear();
    const month = String(now.getUTCMonth() + 1).padStart(2, '0');
    const day = String(now.getUTCDate()).padStart(2, '0');
    const hours = String(now.getUTCHours()).padStart(2, '0');
    const minutes = String(now.getUTCMinutes()).padStart(2, '0');
    
    const formattedDate = `${year}-${month}-${day}T${hours}:${minutes}`;
    document.getElementById('watchTimeDate').value = formattedDate;
    
    // Update both timezone fields
    updateTimezone(1);
    updateTimezone(2);
}

// Function to update timezone code and adjust timestamps
function updateTimezone(zoneNum) {
    const timezoneSelect = document.getElementById(`time${zoneNum}Timezone`);
    const nameInput = document.getElementById(`time${zoneNum}Name`);
    
    // Get selected timezone value (format: "+01:00|CET")
    const selectedValue = timezoneSelect.value;
    const parts = selectedValue.split('|');
    
    // Update the name field with the timezone code
    if (parts.length > 1) {
        nameInput.value = parts[1];
    }
}

// Function to gather all data from the form
function collectFormData() {
    // Time settings
    const includeTime = document.getElementById('includeTime').checked;
    
    // Get the reference time from the datetime-local input (in UTC) or use current UTC time
    let utcReferenceTime = new Date();
    const watchTimeStr = document.getElementById('watchTimeDate').value;
    if (watchTimeStr) {
        // Parse the input as UTC
        utcReferenceTime = new Date(watchTimeStr + 'Z'); // Adding Z to denote UTC
    } else {
        // Set to current UTC time
        utcReferenceTime = new Date(
            Date.UTC(
                utcReferenceTime.getUTCFullYear(),
                utcReferenceTime.getUTCMonth(),
                utcReferenceTime.getUTCDate(),
                utcReferenceTime.getUTCHours(),
                utcReferenceTime.getUTCMinutes(),
                utcReferenceTime.getUTCSeconds()
            )
        );
    }
    
    // Calculate time for timezone 1
    const timezone1Str = document.getElementById('time1Timezone').value.split('|')[0]; // "+01:00"
    let tz1OffsetHours = 0;
    
    if (timezone1Str.includes(':')) {
        const sign = timezone1Str.charAt(0);
        const [hours, minutes] = timezone1Str.substr(1).split(':').map(Number);
        tz1OffsetHours = hours + (minutes / 60);
        if (sign === '-') tz1OffsetHours = -tz1OffsetHours;
    }
    
    // Apply timezone 1 offset (in milliseconds)
    const time1Date = new Date(utcReferenceTime.getTime() + (tz1OffsetHours * 60 * 60 * 1000));
    
    const time1 = {
        zone: 1,
        name: document.getElementById('time1Name').value,
        is24h: document.getElementById('time1Is24h').checked,
        dateFormat: document.getElementById('time1Format').value,
        timestamp: Math.floor(time1Date.getTime() / 1000), // Unix timestamp in seconds
        offsetHours: tz1OffsetHours // Store offset for debugging
    };
    
    // Calculate time for timezone 2
    const timezone2Str = document.getElementById('time2Timezone').value.split('|')[0]; // "+00:00"
    let tz2OffsetHours = 0;
    
    if (timezone2Str.includes(':')) {
        const sign = timezone2Str.charAt(0);
        const [hours, minutes] = timezone2Str.substr(1).split(':').map(Number);
        tz2OffsetHours = hours + (minutes / 60);
        if (sign === '-') tz2OffsetHours = -tz2OffsetHours;
    }
    
    // Apply timezone 2 offset (in milliseconds)
    const time2Date = new Date(utcReferenceTime.getTime() + (tz2OffsetHours * 60 * 60 * 1000));
    
    const time2 = {
        zone: 2,
        name: document.getElementById('time2Name').value,
        is24h: document.getElementById('time2Is24h').checked,
        dateFormat: document.getElementById('time2Format').value,
        timestamp: Math.floor(time2Date.getTime() / 1000), // Unix timestamp in seconds
        offsetHours: tz2OffsetHours // Store offset for debugging
    };
    
    // Alarms
    const includeAlarms = document.getElementById('includeAlarms').checked;
    const alarms = [];
    
    if (includeAlarms) {
        document.querySelectorAll('.alarm-item').forEach(item => {
            alarms.push({
                number: parseInt(item.querySelector('.alarm-number').value),
                audible: item.querySelector('.alarm-audible').checked,
                hour: parseInt(item.querySelector('.alarm-hour').value),
                minute: parseInt(item.querySelector('.alarm-minute').value),
                message: item.querySelector('.alarm-message').value
            });
        });
    }
    
    // EEPROM Data (combined section)
    const includeEeprom = document.getElementById('includeEeprom').checked;
    const appointmentNotification = parseInt(document.getElementById('appointmentNotification').value);
    
    // Appointments
    const appointments = [];
    document.querySelectorAll('.appointment-item').forEach(item => {
        appointments.push({
            date: item.querySelector('.appointment-date').value,
            message: item.querySelector('.appointment-message').value
        });
    });
    
    // Anniversaries
    const anniversaries = [];
    document.querySelectorAll('.anniversary-item').forEach(item => {
        anniversaries.push({
            date: item.querySelector('.anniversary-date').value,
            message: item.querySelector('.anniversary-message').value
        });
    });
    
    // Phone Numbers
    const phoneNumbers = [];
    document.querySelectorAll('.phone-item').forEach(item => {
        phoneNumbers.push({
            name: item.querySelector('.phone-name').value,
            number: item.querySelector('.phone-number').value,
            type: item.querySelector('.phone-type').value
        });
    });
    
    // Lists
    const lists = [];
    document.querySelectorAll('.list-item').forEach(item => {
        lists.push({
            entry: item.querySelector('.list-entry').value,
            priority: parseInt(item.querySelector('.list-priority').value)
        });
    });
    
    // Sound Options
    const includeSoundOptions = document.getElementById('includeSoundOptions').checked;
    const soundOptions = {
        hourlyChime: document.getElementById('hourlyChime').checked,
        buttonBeep: document.getElementById('buttonBeep').checked
    };
    
    // Sound Theme
    const includeSoundTheme = document.getElementById('includeSoundTheme').checked && soundThemeData !== null;
    
    // Wrist App
    const includeWristApp = document.getElementById('includeWristApp').checked && wristAppData !== null;
    
    // Sync Options
    const syncLength = parseInt(document.getElementById('syncLength').value);
    
    return {
        includeTime,
        time1,
        time2,
        includeAlarms,
        alarms,
        includeEeprom,
        appointmentNotification: appointmentNotification >= 0 ? appointmentNotification : null,
        appointments,
        anniversaries,
        phoneNumbers,
        lists,
        includeSoundOptions,
        soundOptions,
        includeSoundTheme,
        soundThemeData: includeSoundTheme ? Array.from(soundThemeData) : null,
        includeWristApp,
        wristAppData: includeWristApp ? Array.from(wristAppData) : null,
        syncLength
    };
}

// Function to send data to the watch
async function sendDataToWatch() {
    if (!port) {
        updateStatus('Not connected to a serial port', true);
        return;
    }
    
    try {
        updateStatus('Preparing data for watch...');
        
        const formData = collectFormData();
        log('Collected form data:', false);
        log(JSON.stringify(formData, null, 2));
        
        // Log timezone information
        if (formData.includeTime) {
            const time1Date = new Date(formData.time1.timestamp * 1000);
            const time2Date = new Date(formData.time2.timestamp * 1000);
            
            log(`Time Zone 1 (${formData.time1.name}): ${time1Date.toISOString()} (UTC${formData.time1.offsetHours >= 0 ? '+' : ''}${formData.time1.offsetHours})`);
            log(`Time Zone 2 (${formData.time2.name}): ${time2Date.toISOString()} (UTC${formData.time2.offsetHours >= 0 ? '+' : ''}${formData.time2.offsetHours})`);
        }
        
        log('Generating packets...');
        
        // Pass the form data to the WebAssembly module
        const packets = wasmModule.generate_protocol3_packets(formData);
        
        log(`Generated ${packets.length} packets`);
        
        updateStatus('Sending data to watch...');
        
        // Get a writer for the serial port
        const writer = port.writable.getWriter();
        
        // Loop through each packet
        for (let i = 0; i < packets.length; i++) {
            const packet = packets[i];
            log(`Sending packet ${i+1} of ${packets.length} (${packet.length} bytes)`);
            
            // Send each byte with a delay
            for (let j = 0; j < packet.length; j++) {
                await writer.write(new Uint8Array([packet[j]]));
                // Sleep between bytes (14ms - as in the example)
                await new Promise(resolve => setTimeout(resolve, 14));
            }
            
            // Sleep between packets (80ms - as in the example)
            await new Promise(resolve => setTimeout(resolve, 80));
        }
        
        // Release the writer
        writer.releaseLock();
        
        updateStatus('Data sent successfully', false, true);
        log('All data packets sent to the watch');
        
    } catch (error) {
        updateStatus('Error sending data to watch', true);
        log(`Error sending data: ${error.message}`, true);
        console.error(error);
    }
}

// Function to create an alarm form row
function createAlarmRow(alarm) {
    const row = document.createElement('div');
    row.className = 'item-row alarm-item';
    
    const numberLabel = document.createElement('span');
    numberLabel.textContent = 'Number:';
    
    const numberInput = document.createElement('input');
    numberInput.type = 'number';
    numberInput.min = '1';
    numberInput.max = '5';
    numberInput.value = alarm.number;
    numberInput.className = 'alarm-number';
    numberInput.style.width = '50px';
    
    const hourLabel = document.createElement('span');
    hourLabel.textContent = 'Time:';
    
    const hourInput = document.createElement('input');
    hourInput.type = 'number';
    hourInput.min = '0';
    hourInput.max = '23';
    hourInput.value = alarm.hour;
    hourInput.className = 'alarm-hour';
    hourInput.style.width = '50px';
    
    const timeColon = document.createElement('span');
    timeColon.textContent = ':';
    
    const minuteInput = document.createElement('input');
    minuteInput.type = 'number';
    minuteInput.min = '0';
    minuteInput.max = '59';
    minuteInput.value = alarm.minute;
    minuteInput.className = 'alarm-minute';
    minuteInput.style.width = '50px';
    
    const messageLabel = document.createElement('span');
    messageLabel.textContent = 'Message:';
    
    const messageInput = document.createElement('input');
    messageInput.type = 'text';
    messageInput.value = alarm.message;
    messageInput.className = 'alarm-message';
    messageInput.maxLength = 8;
    
    const audibleLabel = document.createElement('span');
    audibleLabel.textContent = 'Audible:';
    
    const audibleToggle = document.createElement('label');
    audibleToggle.className = 'toggle-switch';
    
    const audibleInput = document.createElement('input');
    audibleInput.type = 'checkbox';
    audibleInput.checked = alarm.audible;
    audibleInput.className = 'alarm-audible';
    
    const sliderSpan = document.createElement('span');
    sliderSpan.className = 'toggle-slider';
    
    audibleToggle.appendChild(audibleInput);
    audibleToggle.appendChild(sliderSpan);
    
    const removeButton = document.createElement('button');
    removeButton.className = 'danger';
    removeButton.textContent = 'Remove';
    removeButton.onclick = function() {
        row.remove();
    };
    
    row.appendChild(numberLabel);
    row.appendChild(numberInput);
    row.appendChild(hourLabel);
    row.appendChild(hourInput);
    row.appendChild(timeColon);
    row.appendChild(minuteInput);
    row.appendChild(messageLabel);
    row.appendChild(messageInput);
    row.appendChild(audibleLabel);
    row.appendChild(audibleToggle);
    row.appendChild(removeButton);
    
    return row;
}

// Function to create an appointment form row
function createAppointmentRow(appointment) {
    const row = document.createElement('div');
    row.className = 'item-row appointment-item';
    
    const dateLabel = document.createElement('span');
    dateLabel.textContent = 'Date & Time:';
    
    const dateInput = document.createElement('input');
    dateInput.type = 'datetime-local';
    dateInput.value = appointment.date;
    dateInput.className = 'appointment-date';
    
    const messageLabel = document.createElement('span');
    messageLabel.textContent = 'Message:';
    
    const messageInput = document.createElement('input');
    messageInput.type = 'text';
    messageInput.value = appointment.message;
    messageInput.className = 'appointment-message';
    messageInput.maxLength = 12;
    
    const removeButton = document.createElement('button');
    removeButton.className = 'danger';
    removeButton.textContent = 'Remove';
    removeButton.onclick = function() {
        row.remove();
    };
    
    row.appendChild(dateLabel);
    row.appendChild(dateInput);
    row.appendChild(messageLabel);
    row.appendChild(messageInput);
    row.appendChild(removeButton);
    
    return row;
}

// Function to create an anniversary form row
function createAnniversaryRow(anniversary) {
    const row = document.createElement('div');
    row.className = 'item-row anniversary-item';
    
    const dateLabel = document.createElement('span');
    dateLabel.textContent = 'Date:';
    
    const dateInput = document.createElement('input');
    dateInput.type = 'date';
    dateInput.value = anniversary.date;
    dateInput.className = 'anniversary-date';
    
    const messageLabel = document.createElement('span');
    messageLabel.textContent = 'Description:';
    
    const messageInput = document.createElement('input');
    messageInput.type = 'text';
    messageInput.value = anniversary.message;
    messageInput.className = 'anniversary-message';
    messageInput.maxLength = 12;
    
    const removeButton = document.createElement('button');
    removeButton.className = 'danger';
    removeButton.textContent = 'Remove';
    removeButton.onclick = function() {
        row.remove();
    };
    
    row.appendChild(dateLabel);
    row.appendChild(dateInput);
    row.appendChild(messageLabel);
    row.appendChild(messageInput);
    row.appendChild(removeButton);
    
    return row;
}

// Function to create a phone number form row
function createPhoneNumberRow(phone) {
    const row = document.createElement('div');
    row.className = 'item-row phone-item';
    
    const nameLabel = document.createElement('span');
    nameLabel.textContent = 'Name:';
    
    const nameInput = document.createElement('input');
    nameInput.type = 'text';
    nameInput.value = phone.name;
    nameInput.className = 'phone-name';
    nameInput.maxLength = 12;
    
    const numberLabel = document.createElement('span');
    numberLabel.textContent = 'Number:';
    
    const numberInput = document.createElement('input');
    numberInput.type = 'text';
    numberInput.value = phone.number;
    numberInput.className = 'phone-number';
    numberInput.maxLength = 20;
    
    const typeLabel = document.createElement('span');
    typeLabel.textContent = 'Type:';
    
    const typeInput = document.createElement('select');
    typeInput.className = 'phone-type';
    
    const typeOptions = [
        { value: "H", text: "Home" },
        { value: "W", text: "Work" },
        { value: "C", text: "Cell" },
        { value: "F", text: "Fax" },
        { value: "P", text: "Pager" },
        { value: "O", text: "Other" }
    ];
    
    typeOptions.forEach(option => {
        const optionElement = document.createElement('option');
        optionElement.value = option.value;
        optionElement.textContent = option.text;
        if (option.value === phone.type) {
            optionElement.selected = true;
        }
        typeInput.appendChild(optionElement);
    });
    
    const removeButton = document.createElement('button');
    removeButton.className = 'danger';
    removeButton.textContent = 'Remove';
    removeButton.onclick = function() {
        row.remove();
    };
    
    row.appendChild(nameLabel);
    row.appendChild(nameInput);
    row.appendChild(numberLabel);
    row.appendChild(numberInput);
    row.appendChild(typeLabel);
    row.appendChild(typeInput);
    row.appendChild(removeButton);
    
    return row;
}

// Function to create a list item form row
function createListRow(list) {
    const row = document.createElement('div');
    row.className = 'item-row list-item';
    
    const entryLabel = document.createElement('span');
    entryLabel.textContent = 'Entry:';
    
    const entryInput = document.createElement('input');
    entryInput.type = 'text';
    entryInput.value = list.entry;
    entryInput.className = 'list-entry';
    entryInput.maxLength = 12;
    
    const priorityLabel = document.createElement('span');
    priorityLabel.textContent = 'Priority:';
    
    const priorityInput = document.createElement('select');
    priorityInput.className = 'list-priority';
    
    const priorityOptions = [
        { value: "1", text: "1 - Highest" },
        { value: "2", text: "2 - High" },
        { value: "3", text: "3 - Medium" },
        { value: "4", text: "4 - Low" },
        { value: "5", text: "5 - Lowest" },
        { value: "-1", text: "None" }
    ];
    
    priorityOptions.forEach(option => {
        const optionElement = document.createElement('option');
        optionElement.value = option.value;
        optionElement.textContent = option.text;
        if (parseInt(option.value) === list.priority) {
            optionElement.selected = true;
        }
        priorityInput.appendChild(optionElement);
    });
    
    const removeButton = document.createElement('button');
    removeButton.className = 'danger';
    removeButton.textContent = 'Remove';
    removeButton.onclick = function() {
        row.remove();
    };
    
    row.appendChild(entryLabel);
    row.appendChild(entryInput);
    row.appendChild(priorityLabel);
    row.appendChild(priorityInput);
    row.appendChild(removeButton);
    
    return row;
}

// Function to populate form with default data
function populateForm() {
    // Populate Alarms
    const alarmsList = document.getElementById('alarmsList');
    defaultAlarms.forEach(alarm => {
        alarmsList.appendChild(createAlarmRow(alarm));
    });
    
    // Populate Appointments
    const appointmentsList = document.getElementById('appointmentsList');
    defaultAppointments.forEach(appointment => {
        appointmentsList.appendChild(createAppointmentRow(appointment));
    });
    
    // Populate Anniversaries
    const anniversariesList = document.getElementById('anniversariesList');
    defaultAnniversaries.forEach(anniversary => {
        anniversariesList.appendChild(createAnniversaryRow(anniversary));
    });
    
    // Populate Phone Numbers
    const phoneNumbersList = document.getElementById('phoneNumbersList');
    defaultPhoneNumbers.forEach(phone => {
        phoneNumbersList.appendChild(createPhoneNumberRow(phone));
    });
    
    // Populate Lists
    const listsList = document.getElementById('listsList');
    defaultLists.forEach(list => {
        listsList.appendChild(createListRow(list));
    });
}

// Global variables to store file data
let soundThemeData = null;
let wristAppData = null;

// Function to handle sound theme file upload
function handleSoundThemeFileUpload(event) {
    const file = event.target.files[0];
    if (!file) {
        document.getElementById('soundThemeInfo').textContent = 'No file selected';
        soundThemeData = null;
        return;
    }
    
    // Update info text
    document.getElementById('soundThemeInfo').textContent = `File: ${file.name} (${file.size} bytes)`;
    
    // Read the file as ArrayBuffer
    const reader = new FileReader();
    reader.onload = function(e) {
        const arrayBuffer = e.target.result;
        // Convert ArrayBuffer to Uint8Array
        soundThemeData = new Uint8Array(arrayBuffer);
        log(`Sound Theme file loaded: ${file.name} (${soundThemeData.length} bytes)`);
        
        // Enable the include checkbox
        document.getElementById('includeSoundTheme').checked = true;
    };
    reader.onerror = function() {
        log('Error reading Sound Theme file', true);
        soundThemeData = null;
        document.getElementById('soundThemeInfo').textContent = 'Error reading file';
    };
    reader.readAsArrayBuffer(file);
}

// Function to handle wrist app file upload
function handleWristAppFileUpload(event) {
    const file = event.target.files[0];
    if (!file) {
        document.getElementById('wristAppInfo').textContent = 'No file selected';
        wristAppData = null;
        return;
    }
    
    // Update info text
    document.getElementById('wristAppInfo').textContent = `File: ${file.name} (${file.size} bytes)`;
    
    // Read the file as ArrayBuffer
    const reader = new FileReader();
    reader.onload = function(e) {
        const arrayBuffer = e.target.result;
        // Convert ArrayBuffer to Uint8Array
        wristAppData = new Uint8Array(arrayBuffer);
        log(`Wrist App file loaded: ${file.name} (${wristAppData.length} bytes)`);
        
        // Enable the include checkbox
        document.getElementById('includeWristApp').checked = true;
    };
    reader.onerror = function() {
        log('Error reading Wrist App file', true);
        wristAppData = null;
        document.getElementById('wristAppInfo').textContent = 'Error reading file';
    };
    reader.readAsArrayBuffer(file);
}

// Initialize the page
async function initializePage() {
    // Check if Web Serial API is supported
    if (!navigator.serial) {
        updateStatus('Web Serial API is not supported in this browser', true);
        log('Your browser does not support the Web Serial API. Try using Chrome or Edge.', true);
        return;
    }
    
    try {
        // Initialize WebAssembly
        await initWasm();
        
        // Enable connection button
        document.getElementById('connectButton').disabled = false;
        updateStatus('Ready to connect', false, false);
        
        // Set current time in the datetime-local input
        setCurrentTime();
        
        // Add event listeners
        document.getElementById('connectButton').addEventListener('click', connectToSerialPort);
        document.getElementById('sendDataButton').addEventListener('click', sendDataToWatch);
        document.getElementById('disconnectButton').addEventListener('click', disconnectFromSerialPort);
        document.getElementById('setCurrentTimeButton').addEventListener('click', setCurrentTime);
        
        // Add file upload event listeners
        document.getElementById('soundThemeFile').addEventListener('change', handleSoundThemeFileUpload);
        document.getElementById('wristAppFile').addEventListener('change', handleWristAppFileUpload);
        
        // Function to clear all items from a section
        function clearSection(sectionId) {
            const section = document.getElementById(sectionId);
            if (section) {
                while (section.firstChild) {
                    section.removeChild(section.firstChild);
                }
                log(`Cleared all items from ${sectionId}`);
            }
        }

        // Add event listeners for "Add" buttons
        document.getElementById('addAlarmButton').addEventListener('click', () => {
            const newAlarm = { number: 1, audible: true, hour: 9, minute: 0, message: "" };
            document.getElementById('alarmsList').appendChild(createAlarmRow(newAlarm));
        });
        
        document.getElementById('addAppointmentButton').addEventListener('click', () => {
            const today = new Date();
            const formattedDate = today.toISOString().substring(0, 16);
            const newAppointment = { date: formattedDate, message: "" };
            document.getElementById('appointmentsList').appendChild(createAppointmentRow(newAppointment));
        });
        
        document.getElementById('addAnniversaryButton').addEventListener('click', () => {
            const today = new Date();
            const formattedDate = today.toISOString().substring(0, 10);
            const newAnniversary = { date: formattedDate, message: "" };
            document.getElementById('anniversariesList').appendChild(createAnniversaryRow(newAnniversary));
        });
        
        document.getElementById('addPhoneNumberButton').addEventListener('click', () => {
            const newPhone = { name: "", number: "", type: "H" };
            document.getElementById('phoneNumbersList').appendChild(createPhoneNumberRow(newPhone));
        });
        
        document.getElementById('addListButton').addEventListener('click', () => {
            const newList = { entry: "", priority: 3 };
            document.getElementById('listsList').appendChild(createListRow(newList));
        });
        
        // Add event listeners for "Clear" buttons
        document.getElementById('clearAlarmsButton').addEventListener('click', () => {
            clearSection('alarmsList');
        });
        
        document.getElementById('clearAppointmentsButton').addEventListener('click', () => {
            clearSection('appointmentsList');
        });
        
        document.getElementById('clearAnniversariesButton').addEventListener('click', () => {
            clearSection('anniversariesList');
        });
        
        document.getElementById('clearPhoneNumbersButton').addEventListener('click', () => {
            clearSection('phoneNumbersList');
        });
        
        document.getElementById('clearListsButton').addEventListener('click', () => {
            clearSection('listsList');
        });
        
        // Add event listener for "Clear All EEPROM" button
        document.getElementById('clearAllEepromButton').addEventListener('click', () => {
            clearSection('appointmentsList');
            clearSection('anniversariesList');
            clearSection('phoneNumbersList');
            clearSection('listsList');
            log('Cleared all EEPROM data');
        });
        
        // Populate the form with default data
        populateForm();
        
        log('Interface initialized successfully');
        
    } catch (error) {
        updateStatus('Failed to initialize', true);
        log(`Initialization error: ${error.message}`, true);
        console.error(error);
    }
}

// Function to toggle section visibility
function toggleSection(sectionName) {
    const sectionId = sectionName === 'soundtheme' ? 'soundthemeSection' :
                      sectionName === 'wristapp' ? 'wristappSection' :
                      sectionName === 'sound' ? 'soundSection' :
                      `${sectionName}Section`;
    
    const section = document.getElementById(sectionId);
    
    const checkboxId = sectionName === 'soundtheme' ? 'includeSoundTheme' :
                       sectionName === 'wristapp' ? 'includeWristApp' :
                       sectionName === 'sound' ? 'includeSoundOptions' :
                       `include${sectionName.charAt(0).toUpperCase() + sectionName.slice(1)}`;
    
    const checkbox = document.getElementById(checkboxId);
    
    if (section && checkbox) {
        console.log(`Toggling ${sectionId} with checkbox ${checkboxId}: ${checkbox.checked}`);
        if (checkbox.checked) {
            section.classList.remove('hidden-section');
        } else {
            section.classList.add('hidden-section');
        }
    } else {
        console.log(`Section ${sectionId} or checkbox ${checkboxId} not found`);
    }
}

// Initialize section visibility
function initSectionVisibility() {
    toggleSection('time');
    toggleSection('alarms');
    toggleSection('eeprom');
    toggleSection('sound');
    toggleSection('soundtheme');
    toggleSection('wristapp');
}

// When the page loads
window.addEventListener('load', function() {
    initializePage();
    initSectionVisibility();
});
