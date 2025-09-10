let currentApiKey = localStorage.getItem('apiKey');
let currentUser = localStorage.getItem('userEmail');

function showSection(sectionId, buttonElement) {
    document.querySelectorAll('.section').forEach(section => {
        section.classList.remove('active');
    });
    document.querySelectorAll('.nav-btn').forEach(btn => {
        btn.classList.remove('active');
    });

    document.getElementById(sectionId).classList.add('active');
    if (buttonElement) {
        buttonElement.classList.add('active');
    }
}

function showAuthTab(tabId, buttonElement) {
    document.querySelectorAll('.auth-form').forEach(form => {
        form.classList.remove('active');
    });
    document.querySelectorAll('.auth-tab').forEach(tab => {
        tab.classList.remove('active');
    });

    document.getElementById(tabId + '-form').classList.add('active');
    buttonElement.classList.add('active');
}

document.getElementById('generate-prompt').addEventListener('input', function() {
    const count = this.value.length;
    document.getElementById('char-count').textContent = count;
});

document.getElementById('generate-video-prompt').addEventListener('input', function() {
    const count = this.value.length;
    document.getElementById('video-char-count').textContent = count;
});

function setPrompt(text) {
    document.getElementById('generate-prompt').value = text;
    document.getElementById('generate-prompt').dispatchEvent(new Event('input'));
}

function setEditPrompt(text) {
    document.getElementById('edit-prompt').value = text;
}

function setVideoPrompt(text) {
    document.getElementById('generate-video-prompt').value = text;
    document.getElementById('generate-video-prompt').dispatchEvent(new Event('input'));
}

function setEditVideoPrompt(text) {
    document.getElementById('edit-video-prompt').value = text;
}

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

function handleVideoFileSelect(event) {
    const file = event.target.files[0];
    if (file) {
        const reader = new FileReader();
        reader.onload = function(e) {
            document.getElementById('edit-video-preview').src = e.target.result;
            document.getElementById('edit-video-preview').style.display = 'block';
            document.getElementById('original-video-image').src = e.target.result;
            document.getElementById('edit-video-btn').disabled = false;
        };
        reader.readAsDataURL(file);
    }
}

const videoDropZone = document.getElementById('video-file-drop-zone');
videoDropZone.addEventListener('dragover', (e) => {
    e.preventDefault();
    videoDropZone.classList.add('dragover');
});
videoDropZone.addEventListener('dragleave', () => {
    videoDropZone.classList.remove('dragover');
});
videoDropZone.addEventListener('drop', (e) => {
    e.preventDefault();
    videoDropZone.classList.remove('dragover');
    const files = e.dataTransfer.files;
    if (files.length > 0) {
        document.getElementById('edit-video-file').files = files;
        handleVideoFileSelect({ target: { files: files } });
    }
});

async function handleLogin(event) {
    event.preventDefault();

    const email = document.getElementById('login-email').value;
    const password = document.getElementById('login-password').value;

    const statusElement = document.getElementById('login-status');
    statusElement.textContent = 'Logging in...';
    statusElement.className = 'auth-status';

    try {
        const response = await fetch('/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ email, password }),
        });

        const data = await response.json();

        if (data.success) {
            currentApiKey = data.api_key;
            currentUser = email;
            localStorage.setItem('apiKey', currentApiKey);
            localStorage.setItem('userEmail', currentUser);

            updateAuthUI();
            showStatus('login-status', 'Login successful!', 'success');
        } else {
            showStatus('login-status', data.error || 'Login failed', 'error');
        }
    } catch (error) {
        showStatus('login-status', 'Network error. Please try again.', 'error');
    }
}

async function handleRegister(event) {
    event.preventDefault();

    const email = document.getElementById('register-email').value;
    const password = document.getElementById('register-password').value;
    const confirmPassword = document.getElementById('register-confirm-password').value;

    if (password !== confirmPassword) {
        showStatus('register-status', 'Passwords do not match', 'error');
        return;
    }

    const statusElement = document.getElementById('register-status');
    statusElement.textContent = 'Registering...';
    statusElement.className = 'auth-status';

    try {
        const response = await fetch('/register', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ email, password }),
        });

        const data = await response.json();

        if (data.success) {
            currentApiKey = data.api_key;
            currentUser = email;
            localStorage.setItem('apiKey', currentApiKey);
            localStorage.setItem('userEmail', currentUser);

            updateAuthUI();
            showStatus('register-status', 'Registration successful!', 'success');
        } else {
            showStatus('register-status', data.error || 'Registration failed', 'error');
        }
    } catch (error) {
        showStatus('register-status', 'Network error. Please try again.', 'error');
    }
}

function handleLogout() {
    currentApiKey = null;
    currentUser = null;
    localStorage.removeItem('apiKey');
    localStorage.removeItem('userEmail');
    updateAuthUI();
}

function updateAuthUI() {
    const userInfo = document.getElementById('user-info');
    const logoutBtn = document.getElementById('logout-btn');
    const nav = document.getElementById('main-nav');

    if (currentUser && currentApiKey) {
        userInfo.innerHTML = `<strong>Logged in as:</strong> ${currentUser}`;
        logoutBtn.style.display = 'inline-block';
        nav.style.display = 'flex';
        showSection('generate');
    } else {
        userInfo.innerHTML = '<em>Not logged in</em>';
        logoutBtn.style.display = 'none';
        nav.style.display = 'none';
        showSection('auth');
    }
}

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

    if (!currentApiKey) {
        showStatus('generate-status', 'Please login first', 'error');
        showSection('auth');
        return;
    }

    try {
        const response = await fetch('/generate', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${currentApiKey}`,
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

    if (!currentApiKey) {
        showStatus('edit-status', 'Please login first', 'error');
        showSection('auth');
        return;
    }

    try {
        const file = fileInput.files[0];
        const base64 = await fileToBase64(file);

        const response = await fetch('/edit', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${currentApiKey}`,
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

async function generateVideo() {
    const prompt = document.getElementById('generate-video-prompt').value.trim();
    const negativePrompt = document.getElementById('generate-video-negative').value.trim();
    const aspectRatio = document.getElementById('video-aspect-ratio').value;
    const resolution = document.getElementById('video-resolution').value;

    if (!prompt) {
        showStatus('generate-video-status', 'Please enter a prompt', 'error');
        return;
    }

    const btn = document.getElementById('generate-video-btn');
    const loading = document.getElementById('generate-video-loading');
    btn.disabled = true;
    loading.style.display = 'inline-block';

    if (!currentApiKey) {
        showStatus('generate-video-status', 'Please login first', 'error');
        showSection('auth');
        return;
    }

    try {
        const response = await fetch('/generate_video', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${currentApiKey}`,
            },
            body: JSON.stringify({
                prompt,
                negative_prompt: negativePrompt || undefined,
                aspect_ratio: aspectRatio,
                resolution: resolution
            }),
        });

        const data = await response.json();

        if (data.success) {
            pollVideoStatus(data.operation_name, 'generate-video');
        } else {
            showStatus('generate-video-status', data.error || 'Failed to start video generation', 'error');
        }
    } catch (error) {
        showStatus('generate-video-status', 'Network error. Please try again.', 'error');
    } finally {
        btn.disabled = false;
        loading.style.display = 'none';
    }
}

async function editVideo() {
    const fileInput = document.getElementById('edit-video-file');
    const prompt = document.getElementById('edit-video-prompt').value.trim();
    const negativePrompt = document.getElementById('edit-video-negative').value.trim();
    const aspectRatio = document.getElementById('edit-video-aspect-ratio').value;
    const resolution = document.getElementById('edit-video-resolution').value;

    if (!fileInput.files[0]) {
        showStatus('edit-video-status', 'Please select an image', 'error');
        return;
    }

    if (!prompt) {
        showStatus('edit-video-status', 'Please enter an edit prompt', 'error');
        return;
    }

    const btn = document.getElementById('edit-video-btn');
    const loading = document.getElementById('edit-video-loading');
    btn.disabled = true;
    loading.style.display = 'inline-block';

    if (!currentApiKey) {
        showStatus('edit-video-status', 'Please login first', 'error');
        showSection('auth');
        return;
    }

    try {
        const file = fileInput.files[0];
        const base64 = await fileToBase64(file);

        const response = await fetch('/edit_video', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${currentApiKey}`,
            },
            body: JSON.stringify({
                prompt,
                image: base64,
                mime_type: file.type || 'image/jpeg',
                negative_prompt: negativePrompt || undefined,
                aspect_ratio: aspectRatio,
                resolution: resolution
            }),
        });

        const data = await response.json();

        if (data.success) {
            pollVideoStatus(data.operation_name, 'edit-video');
        } else {
            showStatus('edit-video-status', data.error || 'Failed to start video editing', 'error');
        }
    } catch (error) {
        showStatus('edit-video-status', 'Network error. Please try again.', 'error');
    } finally {
        btn.disabled = false;
        loading.style.display = 'none';
    }
}

async function pollVideoStatus(operationName, section) {
    const resultContainer = document.getElementById(`${section}-result`);
    const videoElement = document.getElementById(section === 'generate-video' ? 'generated-video' : 'edited-video');
    const downloadBtn = document.getElementById(section === 'generate-video' ? 'download-generate-video-btn' : 'download-edit-video-btn');

    showStatus(`${section}-status`, 'Video generation in progress...', 'info');

    const pollInterval = setInterval(async () => {
        try {
            const response = await fetch(`/video_status/${operationName}`, {
                headers: {
                    'Authorization': `Bearer ${currentApiKey}`,
                },
            });
            const data = await response.json();

            if (data.success && data.done) {
                clearInterval(pollInterval);

                if (data.video_uri) {
                    videoElement.src = data.video_uri;
                    videoElement.style.display = 'block';
                    resultContainer.style.display = 'block';
                    downloadBtn.style.display = 'inline-block';

                    if (section === 'edit-video') {
                        document.getElementById('video-comparison-container').style.display = 'flex';
                    }

                    showStatus(`${section}-status`, 'Video generated successfully!', 'success');
                } else {
                    showStatus(`${section}-status`, 'Video generation completed but no download available', 'error');
                }
            } else if (!data.success) {
                clearInterval(pollInterval);
                showStatus(`${section}-status`, data.error || 'Video generation failed', 'error');
            }
        } catch (error) {
            clearInterval(pollInterval);
            showStatus(`${section}-status`, 'Network error during polling', 'error');
        }
    }, 10000);
}

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

function downloadVideo(videoId) {
    const video = document.getElementById(videoId);
    const link = document.createElement('a');
    link.href = video.src;
    link.download = `gemini-video-${Date.now()}.mp4`;
    link.click();
}

async function checkHealth() {
    try {
        const response = await fetch('/health');
        const data = await response.json();
        console.log('API Health:', data);
    } catch (error) {
        console.log('Health check failed:', error);
    }
}

document.addEventListener('DOMContentLoaded', function() {
    checkHealth();
    updateAuthUI();
});