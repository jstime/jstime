// Polyfill for Date locale methods
// Replaces the broken native ICU-based implementations with working JS versions

(function() {
    'use strict';
    
    // Helper to format number with leading zero
    function pad(n) {
        return n < 10 ? '0' + n : '' + n;
    }
    
    // Month names
    const monthNames = ['January', 'February', 'March', 'April', 'May', 'June',
                       'July', 'August', 'September', 'October', 'November', 'December'];
    const monthNamesShort = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun',
                            'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
    
    // Day names
    const dayNames = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];
    const dayNamesShort = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
    
    // Basic toLocaleString implementation
    // Format: MM/DD/YYYY, HH:MM:SS AM/PM
    Date.prototype.toLocaleString = function(locales, options) {
        if (isNaN(this.getTime())) {
            return 'Invalid Date';
        }
        
        const month = pad(this.getMonth() + 1);
        const date = pad(this.getDate());
        const year = this.getFullYear();
        const hours = this.getHours();
        const minutes = pad(this.getMinutes());
        const seconds = pad(this.getSeconds());
        
        // Convert to 12-hour format
        const hour12 = hours % 12 || 12;
        const ampm = hours >= 12 ? 'PM' : 'AM';
        
        return `${month}/${date}/${year}, ${hour12}:${minutes}:${seconds} ${ampm}`;
    };
    
    // Basic toLocaleDateString implementation
    // Format: MM/DD/YYYY
    Date.prototype.toLocaleDateString = function(locales, options) {
        if (isNaN(this.getTime())) {
            return 'Invalid Date';
        }
        
        const month = pad(this.getMonth() + 1);
        const date = pad(this.getDate());
        const year = this.getFullYear();
        
        return `${month}/${date}/${year}`;
    };
    
    // Basic toLocaleTimeString implementation
    // Format: HH:MM:SS AM/PM
    Date.prototype.toLocaleTimeString = function(locales, options) {
        if (isNaN(this.getTime())) {
            return 'Invalid Date';
        }
        
        const hours = this.getHours();
        const minutes = pad(this.getMinutes());
        const seconds = pad(this.getSeconds());
        
        // Convert to 12-hour format
        const hour12 = hours % 12 || 12;
        const ampm = hours >= 12 ? 'PM' : 'AM';
        
        return `${hour12}:${minutes}:${seconds} ${ampm}`;
    };
})();
