// Navigation
function showSection(sectionId, buttonElement) {
    // Hide all sections
    document.querySelectorAll('.section').forEach(section => {
        section.classList.remove('active');
    });
    document.querySelectorAll('.nav-btn').forEach(btn => {
        btn.classList.remove('active');
    });

    // Show selected section
    document.getElementById(sectionId).classList.add('active');
    if (buttonElement) {
        buttonElement.classList.add('active');
    }
}

// Character count for generate prompt
document.getElementById('generate-prompt').addEventListener('input', function() {
    const count = this.value.length;
    document.getElementById('char-count').textContent = count;
});

// Set prompt from examples
function setPrompt(text) {
    document.getElementById('generate-prompt').value = text;
    document.getElementById('generate-prompt').dispatchEvent(new Event('input'));
}

// Set edit prompt from examples
function setEditPrompt(text) {
    document.getElementById('edit-prompt').value = text;
}

// File handling for edit section
function handleFileSelect(event) {
    const file = event.target.files[0];
    if (file) {
        const reader = new FileReader();
        reader.onload = function(e) {
            document.getElementById('edit-preview').src = e.target.result;
            document.getElementById('edit-preview').style.display = 'block';
            document.getElementById('original-image').src = e.target.result;
            document.getElementById('edit-btn').disabled = false;
        };
        reader.readAsDataURL(file);
    }
}

// Drag and drop for file upload
const dropZone = document.getElementById('file-drop-zone');
dropZone.addEventListener('dragover', (e) => {
    e.preventDefault();
    dropZone.classList.add('dragover');
});
dropZone.addEventListener('dragleave', () => {
    dropZone.classList.remove('dragover');
});
dropZone.addEventListener('drop', (e) => {
    e.preventDefault();
    dropZone.classList.remove('dragover');
    const files = e.dataTransfer.files;
    if (files.length > 0) {
        document.getElementById('edit-file').files = files;
        handleFileSelect({ target: { files: files } });
    }
});

// API functions
async function generateImage() {
    const prompt = document.getElementById('generate-prompt').value.trim();
    if (!prompt) {
        showStatus('generate-status', 'Please enter a prompt', 'error');
        return;
    }

    const btn = document.getElementById('generate-btn');
    const loading = document.getElementById('generate-loading');
    btn.disabled = true;
    loading.style.display = 'inline-block';

    try {
        const response = await fetch('/generate', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ prompt }),
        });

        const data = await response.json();

        if (data.success) {
            document.getElementById('generated-image').src = `data:image/png;base64,${data.image}`;
            document.getElementById('generated-image').style.display = 'block';
            document.getElementById('download-generate-btn').style.display = 'inline-block';
            document.getElementById('generate-result').style.display = 'block';
            showStatus('generate-status', 'Image generated successfully!', 'success');
        } else {
            showStatus('generate-status', data.error || 'Failed to generate image', 'error');
        }
    } catch (error) {
        showStatus('generate-status', 'Network error. Please try again.', 'error');
    } finally {
        btn.disabled = false;
        loading.style.display = 'none';
    }
}

async function editImage() {
    const fileInput = document.getElementById('edit-file');
    const prompt = document.getElementById('edit-prompt').value.trim();

    if (!fileInput.files[0]) {
        showStatus('edit-status', 'Please select an image', 'error');
        return;
    }

    if (!prompt) {
        showStatus('edit-status', 'Please enter an edit prompt', 'error');
        return;
    }

    const btn = document.getElementById('edit-btn');
    const loading = document.getElementById('edit-loading');
    btn.disabled = true;
    loading.style.display = 'inline-block';

    try {
        // Convert file to base64
        const file = fileInput.files[0];
        const base64 = await fileToBase64(file);

        const response = await fetch('/edit', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                prompt,
                image: base64,
                mime_type: file.type || 'image/jpeg'
            }),
        });

        const data = await response.json();

        if (data.success) {
            document.getElementById('edited-image').src = `data:image/png;base64,${data.image}`;
            document.getElementById('edit-result').style.display = 'block';
            document.getElementById('comparison-container').style.display = 'flex';
            document.getElementById('download-edit-btn').style.display = 'inline-block';
            showStatus('edit-status', 'Image edited successfully!', 'success');
        } else {
            showStatus('edit-status', data.error || 'Failed to edit image', 'error');
        }
    } catch (error) {
        showStatus('edit-status', 'Network error. Please try again.', 'error');
    } finally {
        btn.disabled = false;
        loading.style.display = 'none';
    }
}

// Utility functions
function showStatus(elementId, message, type) {
    const element = document.getElementById(elementId);
    element.textContent = message;
    element.className = `status ${type}`;
}

function fileToBase64(file) {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.readAsDataURL(file);
        reader.onload = () => {
            // Remove the data:image/jpeg;base64, prefix
            const base64 = reader.result.split(',')[1];
            resolve(base64);
        };
        reader.onerror = reject;
    });
}

function downloadImage(imageId) {
    const img = document.getElementById(imageId);
    const link = document.createElement('a');
    link.href = img.src;
    link.download = `gemini-image-${Date.now()}.png`;
    link.click();
}

// Check API health on load
async function checkHealth() {
    try {
        const response = await fetch('/health');
        const data = await response.json();
        console.log('API Health:', data);
    } catch (error) {
        console.log('Health check failed:', error);
    }
}

// Initialize
document.addEventListener('DOMContentLoaded', function() {
    checkHealth();
});