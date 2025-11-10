use serde::{Deserialize, Serialize};

/// Represents a parent frame reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parent {
    pub id: String,
}

/// Represents a Miro board
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Board {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub created_at: String,
}

/// API response for list boards endpoint
#[derive(Debug, Deserialize)]
pub struct BoardsResponse {
    pub data: Vec<Board>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Request body for creating a board
#[derive(Debug, Serialize)]
pub struct CreateBoardRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Response body for single board creation
#[derive(Debug, Deserialize)]
pub struct CreateBoardResponse {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub created_at: String,
}

/// Position for visual elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}

/// Geometry dimensions for visual elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Geometry {
    pub width: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
}

/// Sticky note data payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickyNoteData {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
}

/// Sticky note style configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickyNoteStyle {
    #[serde(rename = "fillColor")]
    pub fill_color: String,
}

/// Request body for creating a sticky note
#[derive(Debug, Clone, Serialize)]
pub struct CreateStickyNoteRequest {
    pub data: StickyNoteData,
    pub style: StickyNoteStyle,
    pub position: Position,
    pub geometry: Geometry,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Parent>,
}

/// Response for sticky note creation
#[derive(Debug, Clone, Deserialize)]
pub struct StickyNoteResponse {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<StickyNoteData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<StickyNoteStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<Geometry>,
}

/// Shape data payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    pub shape: String,
}

/// Shape style configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeStyle {
    #[serde(rename = "fillColor")]
    pub fill_color: String,
    #[serde(rename = "borderColor", skip_serializing_if = "Option::is_none")]
    pub border_color: Option<String>,
    #[serde(rename = "borderWidth", skip_serializing_if = "Option::is_none")]
    pub border_width: Option<String>,
}

/// Request body for creating a shape
#[derive(Debug, Clone, Serialize)]
pub struct CreateShapeRequest {
    pub data: ShapeData,
    pub style: ShapeStyle,
    pub position: Position,
    pub geometry: Geometry,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Parent>,
}

/// Response for shape creation
#[derive(Debug, Clone, Deserialize)]
pub struct ShapeResponse {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ShapeData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ShapeStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<Geometry>,
}

/// Text data payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextData {
    pub content: String,
}

/// Request body for creating text
#[derive(Debug, Clone, Serialize)]
pub struct CreateTextRequest {
    pub data: TextData,
    pub position: Position,
    pub geometry: Geometry,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Parent>,
}

/// Response for text creation
#[derive(Debug, Clone, Deserialize)]
pub struct TextResponse {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<TextData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<Geometry>,
}

/// Frame data payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameData {
    pub title: String,
    #[serde(rename = "type")]
    pub frame_type: String,
}

/// Frame style configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameStyle {
    #[serde(rename = "fillColor")]
    pub fill_color: String,
}

/// Request body for creating a frame
#[derive(Debug, Clone, Serialize)]
pub struct CreateFrameRequest {
    pub data: FrameData,
    pub style: FrameStyle,
    pub position: Position,
    pub geometry: Geometry,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Parent>,
}

/// Response for frame creation
#[derive(Debug, Clone, Deserialize)]
pub struct FrameResponse {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<FrameData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<FrameStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<Geometry>,
}

/// Connector style configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorStyle {
    #[serde(rename = "strokeColor", skip_serializing_if = "Option::is_none")]
    pub stroke_color: Option<String>,
    #[serde(rename = "strokeWidth", skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f64>,
    #[serde(rename = "startCap", skip_serializing_if = "Option::is_none")]
    pub start_cap: Option<String>,
    #[serde(rename = "endCap", skip_serializing_if = "Option::is_none")]
    pub end_cap: Option<String>,
}

/// Caption for a connector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Caption {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<f64>,
}

/// Request body for creating a connector
#[derive(Debug, Clone, Serialize)]
pub struct CreateConnectorRequest {
    #[serde(rename = "startItem")]
    pub start_item: String,
    #[serde(rename = "endItem")]
    pub end_item: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ConnectorStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captions: Option<Vec<Caption>>,
}

/// Response for connector creation
#[derive(Debug, Clone, Deserialize)]
pub struct ConnectorResponse {
    pub id: String,
    #[serde(rename = "startItem", skip_serializing_if = "Option::is_none")]
    pub start_item: Option<String>,
    #[serde(rename = "endItem", skip_serializing_if = "Option::is_none")]
    pub end_item: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ConnectorStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captions: Option<Vec<Caption>>,
}

/// Generic item response that can represent any item type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    #[serde(rename = "type")]
    pub item_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<Geometry>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "modifiedAt", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Parent>,
}

/// Response for list items endpoint
#[derive(Debug, Deserialize)]
pub struct ItemsResponse {
    pub data: Vec<Item>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Request body for updating an item (partial update)
#[derive(Debug, Serialize)]
pub struct UpdateItemRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<Geometry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Parent>,
}

/// Item definition for bulk creation - supports all item types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkItemRequest {
    /// Sticky note item
    StickyNote {
        #[serde(rename = "type")]
        item_type: String, // must be "sticky_note"
        data: StickyNoteData,
        style: StickyNoteStyle,
        position: Position,
        geometry: Geometry,
        #[serde(skip_serializing_if = "Option::is_none")]
        parent: Option<Parent>,
    },
    /// Shape item
    Shape {
        #[serde(rename = "type")]
        item_type: String, // must be "shape"
        data: ShapeData,
        style: ShapeStyle,
        position: Position,
        geometry: Geometry,
        #[serde(skip_serializing_if = "Option::is_none")]
        parent: Option<Parent>,
    },
    /// Text item
    Text {
        #[serde(rename = "type")]
        item_type: String, // must be "text"
        data: TextData,
        position: Position,
        geometry: Geometry,
        #[serde(skip_serializing_if = "Option::is_none")]
        parent: Option<Parent>,
    },
    /// Frame item
    Frame {
        #[serde(rename = "type")]
        item_type: String, // must be "frame"
        data: FrameData,
        style: FrameStyle,
        position: Position,
        geometry: Geometry,
        #[serde(skip_serializing_if = "Option::is_none")]
        parent: Option<Parent>,
    },
}

/// Request body for bulk creating items
#[derive(Debug, Serialize)]
pub struct BulkCreateRequest {
    pub items: Vec<BulkItemRequest>,
}

/// Response for bulk item creation
#[derive(Debug, Deserialize)]
pub struct BulkCreateResponse {
    pub data: Vec<Item>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_serialization() {
        let board = Board {
            id: "board-123".to_string(),
            name: "Test Board".to_string(),
            description: Some("A test board".to_string()),
            created_at: "2025-01-01T00:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&board).unwrap();
        assert!(json.contains("board-123"));
        assert!(json.contains("Test Board"));
    }

    #[test]
    fn test_board_deserialization() {
        let json = r#"{
            "id": "board-456",
            "name": "Another Board",
            "description": "Test description",
            "created_at": "2025-01-02T00:00:00Z"
        }"#;

        let board: Board = serde_json::from_str(json).unwrap();
        assert_eq!(board.id, "board-456");
        assert_eq!(board.name, "Another Board");
        assert_eq!(board.description, Some("Test description".to_string()));
    }

    #[test]
    fn test_create_board_request() {
        let request = CreateBoardRequest {
            name: "New Board".to_string(),
            description: Some("New board description".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("New Board"));
        assert!(json.contains("New board description"));
    }

    #[test]
    fn test_create_board_request_without_description() {
        let request = CreateBoardRequest {
            name: "Board Without Desc".to_string(),
            description: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(!json.contains("description")); // Should be skipped when None
        assert!(json.contains("Board Without Desc"));
    }

    #[test]
    fn test_sticky_note_request_serialization() {
        let request = CreateStickyNoteRequest {
            data: StickyNoteData {
                content: "<p>Test note</p>".to_string(),
                shape: Some("square".to_string()),
            },
            style: StickyNoteStyle {
                fill_color: "light_yellow".to_string(),
            },
            position: Position {
                x: 100.0,
                y: 200.0,
                origin: Some("center".to_string()),
            },
            geometry: Geometry {
                width: 200.0,
                height: None,
            },
            parent: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("Test note"));
        assert!(json.contains("light_yellow"));
        assert!(json.contains("\"x\":100"));
        assert!(json.contains("\"y\":200"));
    }

    #[test]
    fn test_shape_request_serialization() {
        let request = CreateShapeRequest {
            data: ShapeData {
                content: Some("<p>Shape text</p>".to_string()),
                shape: "rectangle".to_string(),
            },
            style: ShapeStyle {
                fill_color: "light_blue".to_string(),
                border_color: Some("blue".to_string()),
                border_width: Some("2".to_string()),
            },
            position: Position {
                x: 0.0,
                y: 0.0,
                origin: None,
            },
            geometry: Geometry {
                width: 300.0,
                height: Some(150.0),
            },
            parent: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("rectangle"));
        assert!(json.contains("light_blue"));
        assert!(json.contains("blue"));
        assert!(json.contains("\"width\":300"));
    }

    #[test]
    fn test_text_request_serialization() {
        let request = CreateTextRequest {
            data: TextData {
                content: "Plain text content".to_string(),
            },
            position: Position {
                x: 50.0,
                y: 75.0,
                origin: None,
            },
            geometry: Geometry {
                width: 200.0,
                height: None,
            },
            parent: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("Plain text content"));
        assert!(json.contains("\"x\":50"));
    }

    #[test]
    fn test_frame_request_serialization() {
        let request = CreateFrameRequest {
            data: FrameData {
                title: "Frame Title".to_string(),
                frame_type: "frame".to_string(),
            },
            style: FrameStyle {
                fill_color: "light_gray".to_string(),
            },
            position: Position {
                x: 0.0,
                y: 0.0,
                origin: None,
            },
            geometry: Geometry {
                width: 1000.0,
                height: Some(800.0),
            },
            parent: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("Frame Title"));
        assert!(json.contains("light_gray"));
        assert!(json.contains("\"width\":1000"));
        assert!(json.contains("\"height\":800"));
    }

    #[test]
    fn test_sticky_note_response_deserialization() {
        let json = r#"{
            "id": "note-123",
            "data": {
                "content": "<p>Test</p>",
                "shape": "square"
            },
            "style": {
                "fillColor": "light_yellow"
            },
            "position": {
                "x": 100.0,
                "y": 200.0,
                "origin": "center"
            },
            "geometry": {
                "width": 200.0
            }
        }"#;

        let response: StickyNoteResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, "note-123");
        assert!(response.data.is_some());
    }

    #[test]
    fn test_item_deserialization() {
        let json = r#"{
            "id": "item-123",
            "type": "sticky_note",
            "data": {
                "content": "<p>Test item</p>",
                "shape": "square"
            },
            "style": {
                "fillColor": "light_yellow"
            },
            "position": {
                "x": 100.0,
                "y": 200.0
            },
            "geometry": {
                "width": 200.0
            },
            "createdAt": "2025-01-01T10:00:00Z",
            "modifiedAt": "2025-01-02T14:30:00Z"
        }"#;

        let item: Item = serde_json::from_str(json).unwrap();
        assert_eq!(item.id, "item-123");
        assert_eq!(item.item_type, "sticky_note");
        assert!(item.data.is_some());
        assert_eq!(item.created_at, Some("2025-01-01T10:00:00Z".to_string()));
        assert_eq!(item.modified_at, Some("2025-01-02T14:30:00Z".to_string()));
    }

    #[test]
    fn test_items_response_deserialization() {
        let json = r#"{
            "data": [
                {
                    "id": "item-1",
                    "type": "sticky_note"
                },
                {
                    "id": "item-2",
                    "type": "shape"
                }
            ],
            "cursor": "next-cursor-123"
        }"#;

        let response: ItemsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].id, "item-1");
        assert_eq!(response.data[0].item_type, "sticky_note");
        assert_eq!(response.cursor, Some("next-cursor-123".to_string()));
    }

    #[test]
    fn test_update_item_request_serialization() {
        let request = UpdateItemRequest {
            position: Some(Position {
                x: 150.0,
                y: 250.0,
                origin: None,
            }),
            data: None,
            style: None,
            geometry: None,
            parent: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"x\":150"));
        assert!(json.contains("\"y\":250"));
        assert!(!json.contains("data")); // Should be skipped when None
    }

    #[test]
    fn test_connector_style_serialization() {
        let style = ConnectorStyle {
            stroke_color: Some("black".to_string()),
            stroke_width: Some(2.0),
            start_cap: Some("none".to_string()),
            end_cap: Some("arrow".to_string()),
        };

        let json = serde_json::to_string(&style).unwrap();
        assert!(json.contains("\"strokeColor\":\"black\""));
        assert!(json.contains("\"strokeWidth\":2"));
        assert!(json.contains("\"startCap\":\"none\""));
        assert!(json.contains("\"endCap\":\"arrow\""));
    }

    #[test]
    fn test_connector_style_with_defaults() {
        let style = ConnectorStyle {
            stroke_color: None,
            stroke_width: None,
            start_cap: None,
            end_cap: None,
        };

        let json = serde_json::to_string(&style).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_caption_serialization() {
        let caption = Caption {
            content: "Test label".to_string(),
            position: Some(0.5),
        };

        let json = serde_json::to_string(&caption).unwrap();
        assert!(json.contains("\"content\":\"Test label\""));
        assert!(json.contains("\"position\":0.5"));
    }

    #[test]
    fn test_caption_without_position() {
        let caption = Caption {
            content: "Test label".to_string(),
            position: None,
        };

        let json = serde_json::to_string(&caption).unwrap();
        assert!(json.contains("\"content\":\"Test label\""));
        assert!(!json.contains("position"));
    }

    #[test]
    fn test_create_connector_request_serialization() {
        let request = CreateConnectorRequest {
            start_item: "item-1".to_string(),
            end_item: "item-2".to_string(),
            style: Some(ConnectorStyle {
                stroke_color: Some("red".to_string()),
                stroke_width: Some(3.0),
                start_cap: Some("circle".to_string()),
                end_cap: Some("arrow".to_string()),
            }),
            captions: Some(vec![Caption {
                content: "Depends on".to_string(),
                position: Some(0.5),
            }]),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"startItem\":\"item-1\""));
        assert!(json.contains("\"endItem\":\"item-2\""));
        assert!(json.contains("\"strokeColor\":\"red\""));
        assert!(json.contains("\"Depends on\""));
    }

    #[test]
    fn test_create_connector_request_minimal() {
        let request = CreateConnectorRequest {
            start_item: "item-1".to_string(),
            end_item: "item-2".to_string(),
            style: None,
            captions: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"startItem\":\"item-1\""));
        assert!(json.contains("\"endItem\":\"item-2\""));
        assert!(!json.contains("style"));
        assert!(!json.contains("captions"));
    }

    #[test]
    fn test_connector_response_deserialization() {
        let json = r#"{
            "id": "connector-123",
            "startItem": "item-1",
            "endItem": "item-2",
            "style": {
                "strokeColor": "black",
                "strokeWidth": 2.0,
                "endCap": "arrow"
            },
            "captions": [
                {
                    "content": "connects",
                    "position": 0.5
                }
            ]
        }"#;

        let response: ConnectorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, "connector-123");
        assert_eq!(response.start_item, Some("item-1".to_string()));
        assert_eq!(response.end_item, Some("item-2".to_string()));
        assert!(response.style.is_some());
        assert!(response.captions.is_some());
        assert_eq!(response.captions.unwrap()[0].content, "connects");
    }

    #[test]
    fn test_bulk_item_request_sticky_note_serialization() {
        let item = BulkItemRequest::StickyNote {
            item_type: "sticky_note".to_string(),
            data: StickyNoteData {
                content: "<p>Test note</p>".to_string(),
                shape: Some("square".to_string()),
            },
            style: StickyNoteStyle {
                fill_color: "light_yellow".to_string(),
            },
            position: Position {
                x: 100.0,
                y: 200.0,
                origin: Some("center".to_string()),
            },
            geometry: Geometry {
                width: 200.0,
                height: None,
            },
            parent: None,
        };

        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("\"type\":\"sticky_note\""));
        assert!(json.contains("Test note"));
        assert!(json.contains("light_yellow"));
    }

    #[test]
    fn test_bulk_item_request_shape_serialization() {
        let item = BulkItemRequest::Shape {
            item_type: "shape".to_string(),
            data: ShapeData {
                content: Some("<p>Shape</p>".to_string()),
                shape: "rectangle".to_string(),
            },
            style: ShapeStyle {
                fill_color: "light_blue".to_string(),
                border_color: Some("blue".to_string()),
                border_width: Some("2".to_string()),
            },
            position: Position {
                x: 0.0,
                y: 0.0,
                origin: None,
            },
            geometry: Geometry {
                width: 300.0,
                height: Some(150.0),
            },
            parent: None,
        };

        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("\"type\":\"shape\""));
        assert!(json.contains("rectangle"));
        assert!(json.contains("light_blue"));
    }

    #[test]
    fn test_bulk_item_request_text_serialization() {
        let item = BulkItemRequest::Text {
            item_type: "text".to_string(),
            data: TextData {
                content: "Plain text".to_string(),
            },
            position: Position {
                x: 50.0,
                y: 75.0,
                origin: None,
            },
            geometry: Geometry {
                width: 200.0,
                height: None,
            },
            parent: None,
        };

        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("\"type\":\"text\""));
        assert!(json.contains("Plain text"));
    }

    #[test]
    fn test_bulk_item_request_frame_serialization() {
        let item = BulkItemRequest::Frame {
            item_type: "frame".to_string(),
            data: FrameData {
                title: "Frame Title".to_string(),
                frame_type: "frame".to_string(),
            },
            style: FrameStyle {
                fill_color: "light_gray".to_string(),
            },
            position: Position {
                x: 0.0,
                y: 0.0,
                origin: None,
            },
            geometry: Geometry {
                width: 1000.0,
                height: Some(800.0),
            },
            parent: None,
        };

        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("\"type\":\"frame\""));
        assert!(json.contains("Frame Title"));
        assert!(json.contains("light_gray"));
    }

    #[test]
    fn test_bulk_create_request_serialization() {
        let items = vec![
            BulkItemRequest::Text {
                item_type: "text".to_string(),
                data: TextData {
                    content: "Item 1".to_string(),
                },
                position: Position {
                    x: 0.0,
                    y: 0.0,
                    origin: None,
                },
                geometry: Geometry {
                    width: 100.0,
                    height: None,
                },
                parent: None,
            },
            BulkItemRequest::Text {
                item_type: "text".to_string(),
                data: TextData {
                    content: "Item 2".to_string(),
                },
                position: Position {
                    x: 100.0,
                    y: 0.0,
                    origin: None,
                },
                geometry: Geometry {
                    width: 100.0,
                    height: None,
                },
                parent: None,
            },
        ];

        let request = BulkCreateRequest { items };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"items\""));
        assert!(json.contains("Item 1"));
        assert!(json.contains("Item 2"));
    }

    #[test]
    fn test_bulk_create_response_deserialization() {
        let json = r#"{
            "data": [
                {
                    "id": "item-1",
                    "type": "text",
                    "data": {
                        "content": "Item 1"
                    }
                },
                {
                    "id": "item-2",
                    "type": "text",
                    "data": {
                        "content": "Item 2"
                    }
                }
            ]
        }"#;

        let response: BulkCreateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].id, "item-1");
        assert_eq!(response.data[0].item_type, "text");
        assert_eq!(response.data[1].id, "item-2");
        assert_eq!(response.data[1].item_type, "text");
    }

    #[test]
    fn test_item_sorting_by_created_at() {
        // Create items with different creation times
        let json_1 = r#"{
            "id": "item-1",
            "type": "sticky_note",
            "createdAt": "2025-01-01T10:00:00Z",
            "modifiedAt": "2025-01-02T14:30:00Z"
        }"#;

        let json_2 = r#"{
            "id": "item-2",
            "type": "sticky_note",
            "createdAt": "2025-01-01T09:00:00Z",
            "modifiedAt": "2025-01-02T13:30:00Z"
        }"#;

        let json_3 = r#"{
            "id": "item-3",
            "type": "sticky_note",
            "createdAt": "2025-01-01T11:00:00Z",
            "modifiedAt": "2025-01-02T15:30:00Z"
        }"#;

        let mut items: Vec<Item> = vec![
            serde_json::from_str(json_1).unwrap(),
            serde_json::from_str(json_2).unwrap(),
            serde_json::from_str(json_3).unwrap(),
        ];

        // Sort by created_at (oldest to newest)
        items.sort_by(|a, b| {
            let a_time = a.created_at.as_deref().unwrap_or("");
            let b_time = b.created_at.as_deref().unwrap_or("");
            a_time.cmp(b_time)
        });

        // Verify order is oldest to newest
        assert_eq!(items[0].id, "item-2"); // 09:00
        assert_eq!(items[1].id, "item-1"); // 10:00
        assert_eq!(items[2].id, "item-3"); // 11:00
    }

    #[test]
    fn test_item_sorting_by_modified_at() {
        // Create items with different modification times
        let json_1 = r#"{
            "id": "item-1",
            "type": "sticky_note",
            "createdAt": "2025-01-01T10:00:00Z",
            "modifiedAt": "2025-01-02T14:30:00Z"
        }"#;

        let json_2 = r#"{
            "id": "item-2",
            "type": "sticky_note",
            "createdAt": "2025-01-01T09:00:00Z",
            "modifiedAt": "2025-01-02T13:30:00Z"
        }"#;

        let json_3 = r#"{
            "id": "item-3",
            "type": "sticky_note",
            "createdAt": "2025-01-01T11:00:00Z",
            "modifiedAt": "2025-01-02T15:30:00Z"
        }"#;

        let mut items: Vec<Item> = vec![
            serde_json::from_str(json_1).unwrap(),
            serde_json::from_str(json_2).unwrap(),
            serde_json::from_str(json_3).unwrap(),
        ];

        // Sort by modified_at (oldest to newest)
        items.sort_by(|a, b| {
            let a_time = a.modified_at.as_deref().unwrap_or("");
            let b_time = b.modified_at.as_deref().unwrap_or("");
            a_time.cmp(b_time)
        });

        // Verify order is oldest to newest
        assert_eq!(items[0].id, "item-2"); // 13:30
        assert_eq!(items[1].id, "item-1"); // 14:30
        assert_eq!(items[2].id, "item-3"); // 15:30
    }
}
