use tokio;
use webrtc::api::media_engine::{MediaEngine, MIME_TYPE_H264, MIME_TYPE_OPUS};
use webrtc::rtp_transceiver::rtp_codec::{RTCRtpCodecCapability, RTCRtpCodecParameters, RTPCodecType};

#[tokio::main]
async fn main() {
    let mut m = MediaEngine::default();

    m.register_codec( // audio
        RTCRtpCodecParameters {
            capability: RTCRtpCodecCapability {
                mime_type: MIME_TYPE_OPUS.to_owned(),
                ..Default::default()
            },
            payload_type: 120,
            ..Default::default()
        },
        RTPCodecType::Audio
    )?;

    m.register_codec( // video
        RTCRtpCodecParameters {
            capability: RTCRtpCodecCapability {
                mime_type: MIME_TYPE_H264.to_owned(),
                clock_rate: 90000,
                channels: 0,
                sdp_fmtp_line: "".to_owned(),
                rtcp_feedback: vec![]
            },
            payload_type: 96,
            ..Default::default()
        },
        RTPCodecType::Video
    )?;
}
