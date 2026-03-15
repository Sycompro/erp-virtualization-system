/**
 * ERP Virtualization - WebRTC Service
 * Maneja la conexión WebRTC con el cPanel (servidor local)
 * para recibir el stream del escritorio remoto
 */
const WebRTCService = (() => {
    // URL del cPanel (servidor local) — se obtiene de la API
    let _cpanelUrl = localStorage.getItem('erp_cpanel_url') || 'http://localhost:8080';

    let _peerConnection = null;
    let _dataChannel = null;
    let _remoteStream = null;

    // Callbacks
    let _onStreamReady = null;
    let _onStatusChange = null;
    let _onDisconnected = null;

    // ICE Servers (TURN/STUN)
    const ICE_SERVERS = [
        { urls: 'stun:stun.l.google.com:19302' },
        { urls: 'stun:stun1.l.google.com:19302' },
        // TURN server del cPanel — se configura dinámicamente
    ];

    /**
     * Iniciar conexión con una aplicación virtualizada
     */
    async function connect(appType, containerId, options = {}) {
        _updateStatus('starting', 'Iniciando contenedor...');

        try {
            // 1. Solicitar al cPanel que inicie el container
            const startResponse = await fetch(`${_cpanelUrl}/api/containers/start`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${AuthService.token}`
                },
                body: JSON.stringify({
                    app_type: appType,
                    user_id: AuthService.user?.id || 'web-user',
                    session_id: `web-${Date.now()}`
                })
            });

            if (!startResponse.ok) {
                throw new Error('No se pudo iniciar el contenedor');
            }

            const containerInfo = await startResponse.json();
            _updateStatus('connecting', 'Estableciendo conexión WebRTC...');

            // 2. Crear PeerConnection
            _peerConnection = new RTCPeerConnection({
                iceServers: ICE_SERVERS
            });

            // Manejar streams entrantes
            _peerConnection.ontrack = (event) => {
                _remoteStream = event.streams[0];
                if (_onStreamReady) _onStreamReady(_remoteStream);
                _updateStatus('connected', 'Conectado');
            };

            // Manejar ICE candidates
            _peerConnection.onicecandidate = async (event) => {
                if (event.candidate) {
                    try {
                        await fetch(`${_cpanelUrl}/api/webrtc/ice-candidate`, {
                            method: 'POST',
                            headers: {
                                'Content-Type': 'application/json',
                                'Authorization': `Bearer ${AuthService.token}`
                            },
                            body: JSON.stringify({
                                container_id: containerInfo.container_id,
                                candidate: JSON.stringify(event.candidate)
                            })
                        });
                    } catch (e) {
                        console.warn('Error enviando ICE candidate:', e);
                    }
                }
            };

            // Manejar cambios de estado
            _peerConnection.onconnectionstatechange = () => {
                const state = _peerConnection.connectionState;
                switch (state) {
                    case 'connected':
                        _updateStatus('connected', 'Conectado');
                        break;
                    case 'disconnected':
                    case 'failed':
                        _updateStatus('disconnected', 'Desconectado');
                        if (_onDisconnected) _onDisconnected();
                        break;
                    case 'closed':
                        _updateStatus('closed', 'Conexión cerrada');
                        break;
                }
            };

            // Crear canal de datos para input (mouse, teclado)
            _dataChannel = _peerConnection.createDataChannel('input', {
                ordered: true
            });

            _dataChannel.onopen = () => {
                console.log('Canal de datos abierto');
            };

            // 3. Crear oferta SDP
            const offer = await _peerConnection.createOffer({
                offerToReceiveVideo: true,
                offerToReceiveAudio: false
            });
            await _peerConnection.setLocalDescription(offer);

            // 4. Enviar oferta al cPanel
            const offerResponse = await fetch(`${_cpanelUrl}/api/webrtc/offer`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${AuthService.token}`
                },
                body: JSON.stringify({
                    container_id: containerInfo.container_id,
                    sdp_offer: JSON.stringify(offer)
                })
            });

            if (!offerResponse.ok) {
                throw new Error('Error en negociación WebRTC');
            }

            const answerData = await offerResponse.json();

            // 5. Establecer respuesta SDP
            const answer = JSON.parse(answerData.sdp_answer);
            await _peerConnection.setRemoteDescription(
                new RTCSessionDescription(answer)
            );

            _updateStatus('waiting', 'Esperando stream de video...');

        } catch (error) {
            _updateStatus('error', error.message);
            disconnect();
            throw error;
        }
    }

    /**
     * Desconectar
     */
    function disconnect() {
        if (_dataChannel) {
            _dataChannel.close();
            _dataChannel = null;
        }

        if (_peerConnection) {
            _peerConnection.close();
            _peerConnection = null;
        }

        _remoteStream = null;
        _updateStatus('disconnected', 'Desconectado');
    }

    /**
     * Enviar input de mouse
     */
    function sendMouseEvent(type, x, y, button) {
        if (_dataChannel && _dataChannel.readyState === 'open') {
            _dataChannel.send(JSON.stringify({
                type: 'mouse',
                event: type,
                x, y, button
            }));
        }
    }

    /**
     * Enviar input de teclado
     */
    function sendKeyEvent(type, key, code) {
        if (_dataChannel && _dataChannel.readyState === 'open') {
            _dataChannel.send(JSON.stringify({
                type: 'keyboard',
                event: type,
                key, code
            }));
        }
    }

    /**
     * Configurar callbacks
     */
    function onStreamReady(cb) { _onStreamReady = cb; }
    function onStatusChange(cb) { _onStatusChange = cb; }
    function onDisconnected(cb) { _onDisconnected = cb; }

    /**
     * Configurar URL del cPanel
     */
    function setCpanelUrl(url) {
        _cpanelUrl = url;
        localStorage.setItem('erp_cpanel_url', url);
    }

    function _updateStatus(state, message) {
        if (_onStatusChange) _onStatusChange(state, message);
    }

    return {
        connect,
        disconnect,
        sendMouseEvent,
        sendKeyEvent,
        onStreamReady,
        onStatusChange,
        onDisconnected,
        setCpanelUrl,
        get cpanelUrl() { return _cpanelUrl; },
        get isConnected() {
            return _peerConnection?.connectionState === 'connected';
        }
    };
})();
