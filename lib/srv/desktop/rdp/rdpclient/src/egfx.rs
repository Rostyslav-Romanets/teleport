use ironrdp_egfx::client::GraphicsPipelineHandler;
use ironrdp_egfx::pdu::GfxPdu;
use log::debug;

pub(crate) struct TeleportGraphicsPipelineHandler;

impl GraphicsPipelineHandler for TeleportGraphicsPipelineHandler {
    fn on_unhandled_pdu(&mut self, pdu: &GfxPdu) {
        match pdu {
            GfxPdu::WireToSurface1(pdu) => debug!("GRAPHICS PIPELINE: Wire To Surface 1 ({:?})", pdu.codec_id),
            GfxPdu::WireToSurface2(_) => debug!("GRAPHICS PIPELINE Wire To Surface 2"),
            GfxPdu::DeleteEncodingContext(_) => debug!("GRAPHICS PIPELINE Delete Encoding Context"),
            GfxPdu::SolidFill(_) => debug!("GRAPHICS PIPELINE Solid Fill"),
            GfxPdu::SurfaceToSurface(_) => debug!("GRAPHICS PIPELINE Surface To Surface"),
            GfxPdu::SurfaceToCache(_) => debug!("GRAPHICS PIPELINE Surface To Cache"),
            GfxPdu::CacheToSurface(_) => debug!("GRAPHICS PIPELINE Cache To Surface"),
            GfxPdu::EvictCacheEntry(_) => debug!("GRAPHICS PIPELINE Evict Cache Entry"),
            GfxPdu::CreateSurface(_) => debug!("GRAPHICS PIPELINE Create Surface"),
            GfxPdu::DeleteSurface(_) => debug!("GRAPHICS PIPELINE Delete Surface"),
            GfxPdu::StartFrame(_) => debug!("GRAPHICS PIPELINE Start Frame"),
            GfxPdu::EndFrame(_) => debug!("GRAPHICS PIPELINE End Frame"),
            GfxPdu::FrameAcknowledge(_) => debug!("GRAPHICS PIPELINE Frame Acknowledge"),
            GfxPdu::ResetGraphics(_) => debug!("GRAPHICS PIPELINE Reset Graphics"),
            GfxPdu::MapSurfaceToOutput(_) => debug!("GRAPHICS PIPELINE Map Surface To Output"),
            GfxPdu::CacheImportOffer(_) => debug!("GRAPHICS PIPELINE Cache Import Offer"),
            GfxPdu::CacheImportReply(_) => debug!("GRAPHICS PIPELINE Cache Import Reply"),
            GfxPdu::CapabilitiesAdvertise(_) => debug!("GRAPHICS PIPELINE Capabilities Advertise"),
            GfxPdu::CapabilitiesConfirm(_) => debug!("GRAPHICS PIPELINE Capabilities Confirm"),
            GfxPdu::MapSurfaceToWindow(_) => debug!("GRAPHICS PIPELINE Map Surface To Window"),
            GfxPdu::QoeFrameAcknowledge(_) => debug!("GRAPHICS PIPELINE Qoe Frame Acknowledge"),
            GfxPdu::MapSurfaceToScaledOutput(_) => debug!("GRAPHICS PIPELINE Map Surface To Scaled Output"),
            GfxPdu::MapSurfaceToScaledWindow(_) => debug!("GRAPHICS PIPELINE Map Surface To Scaled Window"),
            _ => (),
        }
    }
}